use super::*;

fn geometry_server(cols: u16, rows: u16) -> HeadlessServer {
    let mut server = test_headless_server();
    server.headless_size = (cols, rows);
    server.app.state.headless_size = (cols, rows);
    server.app.state.pane_scrollbars = false;
    server.app.state.pane_gaps = false;
    server.app.state.pane_borders = crate::config::PaneBordersConfig::Off;
    let mut workspace = crate::workspace::Workspace::test_new("headless-geometry");
    workspace.test_add_tab(Some("background"));
    let other = crate::workspace::Workspace::test_new("other-workspace");
    server.app.state.workspaces = vec![workspace, other];
    server.app.state.ensure_test_terminals();
    for terminal_id in server.app.state.terminals.keys() {
        server.app.terminal_runtimes.insert(
            terminal_id.clone(),
            crate::terminal::TerminalRuntime::test_with_screen_bytes(60, 24, b""),
        );
    }
    server.app.state.active = Some(0);
    server.app.state.selected = 0;
    server
}

fn assert_runtime_sizes(server: &HeadlessServer, cols: u16, rows: u16) {
    assert!(server.app.terminal_runtimes.len() >= 3);
    for runtime in server.app.terminal_runtimes.values() {
        assert_eq!(runtime.current_size(), (rows, cols));
        assert_eq!(runtime.terminal_dimensions(), Some((cols, rows)));
    }
}

#[tokio::test]
async fn final_shell_disconnect_restores_configured_headless_geometry() {
    for (cols, rows) in [(120, 40), (144, 48)] {
        let mut server = geometry_server(cols, rows);
        let (control, _render) = connect_test_shell(&mut server, 81, 60, 24);
        let _ = control.recv().unwrap();
        assert_runtime_sizes(&server, 60, 24);
        let terminal = server.app.state.workspaces[0].tabs[0]
            .panes
            .values()
            .next()
            .unwrap()
            .attached_terminal_id
            .to_string();
        let _observer_control = connect_pending_terminal_client_with_control_rx(&mut server, 88);
        assert!(
            server.handle_server_event(ServerEvent::ClientObserveTerminal {
                client_id: 88,
                target: terminal,
            })
        );
        server.handle_server_event(ServerEvent::ClientDisconnected { client_id: 81 });
        assert!(server.clients.contains_key(&88));
        assert_eq!(server.effective_size, (cols, rows));
        assert_eq!(
            server.app.state.view.terminal_area,
            Rect::new(0, 0, cols, rows)
        );
        assert_eq!(server.app.state.estimate_pane_size(), (rows, cols));
        assert_runtime_sizes(&server, cols, rows);
        server.app.state.assert_invariants_for_test();
        shutdown_test_runtimes(&mut server);
    }
}

#[tokio::test]
async fn inactive_shell_retains_geometry_until_actual_disconnect() {
    let mut server = geometry_server(120, 40);
    let (control, _render) = connect_test_shell(&mut server, 82, 60, 24);
    let _ = control.recv().unwrap();
    server.set_client_shell_surface_active(82, false);
    assert_runtime_sizes(&server, 60, 24);
    let (other_control, _other_render) = connect_test_shell(&mut server, 89, 90, 30);
    let _ = other_control.recv().unwrap();
    assert_runtime_sizes(&server, 90, 30);
    server.handle_server_event(ServerEvent::ClientDisconnected { client_id: 89 });
    // The frozen shell is still attached even though there is no active shell.
    assert_runtime_sizes(&server, 90, 30);
    server.handle_server_event(ServerEvent::ClientDisconnected { client_id: 82 });
    assert_runtime_sizes(&server, 120, 40);
    shutdown_test_runtimes(&mut server);
}

#[tokio::test]
async fn headless_geometry_preserves_direct_attach_lock_until_disconnect() {
    let mut server = geometry_server(120, 40);
    let (control, _render) = connect_test_shell(&mut server, 83, 60, 24);
    let _ = control.recv().unwrap();
    let pane = server.app.state.workspaces[0].tabs[0].root_pane;
    let terminal = server.app.state.workspaces[0]
        .terminal_id(pane)
        .unwrap()
        .clone();
    connect_pending_terminal_client(&mut server, 84);
    server.handle_server_event(ServerEvent::ClientAttachTerminal {
        client_id: 84,
        terminal_id: terminal.to_string(),
        takeover: false,
    });
    let attached_size = server
        .app
        .terminal_runtimes
        .get(&terminal)
        .unwrap()
        .current_size();
    server.handle_server_event(ServerEvent::ClientDisconnected { client_id: 83 });
    assert_eq!(
        server
            .app
            .terminal_runtimes
            .get(&terminal)
            .unwrap()
            .current_size(),
        attached_size
    );
    let background = server.app.state.workspaces[0].tabs[1].root_pane;
    assert_eq!(
        server
            .app
            .terminal_runtimes
            .get(
                server.app.state.workspaces[0]
                    .terminal_id(background)
                    .unwrap()
            )
            .unwrap()
            .current_size(),
        (40, 120)
    );
    server.handle_server_event(ServerEvent::ClientDisconnected { client_id: 84 });
    assert_runtime_sizes(&server, 120, 40);
    shutdown_test_runtimes(&mut server);
}

#[tokio::test]
async fn new_headless_tab_uses_configured_geometry_after_phone_disconnect() {
    let mut server = geometry_server(120, 40);
    let (control, _render) = connect_test_shell(&mut server, 85, 60, 24);
    let _ = control.recv().unwrap();
    server.handle_server_event(ServerEvent::ClientDisconnected { client_id: 85 });
    let workspace_id = server.app.public_workspace_id(0);
    let (respond_to, response_rx) = std::sync::mpsc::channel();
    server.handle_api_request_with_shutdown_check(api::ApiRequestMessage {
        request: api::schema::Request {
            id: "new-headless-tab".into(),
            method: api::schema::Method::TabCreate(api::schema::TabCreateParams {
                workspace_id: Some(workspace_id),
                cwd: None,
                focus: false,
                label: None,
                env: HashMap::new(),
            }),
        },
        respond_to,
        response_write_complete: None,
        stream_active: None,
    });
    let response: serde_json::Value = serde_json::from_str(&response_rx.recv().unwrap()).unwrap();
    assert!(response.get("error").is_none(), "{response}");
    let workspace = &server.app.state.workspaces[0];
    assert_eq!(workspace.tabs.len(), 3);
    let pane = workspace.tabs[2].root_pane;
    let runtime = server
        .app
        .terminal_runtimes
        .get(workspace.terminal_id(pane).unwrap())
        .unwrap();
    assert_eq!(runtime.current_size(), (40, 120));
    assert_runtime_sizes(&server, 120, 40);
    // Splitting while detached does not shrink either running terminal to
    // its smaller layout allocation; a later attached client owns exact sizing.
    let source = server.app.state.workspaces[0].tabs[0].root_pane;
    let source_id = server.app.public_pane_id(0, source).unwrap();
    let (respond_to, response_rx) = std::sync::mpsc::channel();
    server.handle_api_request_with_shutdown_check(api::ApiRequestMessage {
        request: api::schema::Request {
            id: "split-headless-pane".into(),
            method: api::schema::Method::PaneSplit(api::schema::PaneSplitParams {
                workspace_id: None,
                target_pane_id: Some(source_id),
                direction: api::schema::SplitDirection::Right,
                ratio: None,
                cwd: None,
                focus: false,
                right_click: api::schema::PaneRightClickTarget::Herdr,
                env: HashMap::new(),
            }),
        },
        respond_to,
        response_write_complete: None,
        stream_active: None,
    });
    let response: serde_json::Value = serde_json::from_str(&response_rx.recv().unwrap()).unwrap();
    assert!(response.get("error").is_none(), "{response}");
    let tab = &server.app.state.workspaces[0].tabs[0];
    assert_eq!(tab.panes.len(), 2);
    for pane in tab.panes.values() {
        let runtime = server
            .app
            .terminal_runtimes
            .get(&pane.attached_terminal_id)
            .unwrap();
        assert_eq!(runtime.current_size(), (40, 120));
    }
    server.app.state.assert_invariants_for_test();
    shutdown_test_runtimes(&mut server);
}

#[tokio::test]
async fn headless_growth_preserves_wide_dimensions_and_skips_unchanged_geometry() {
    for (cols, rows) in [(160, 50), (200, 20), (60, 80), (120, 40)] {
        let mut server = geometry_server(120, 40);
        let (control, _render) = connect_test_shell(&mut server, 91, cols, rows);
        let _ = control.recv().unwrap();
        let before: Vec<_> = server
            .app
            .state
            .terminals
            .keys()
            .map(|id| {
                let runtime = server.app.terminal_runtimes.get(id).unwrap();
                (id.clone(), runtime.content_seq())
            })
            .collect();
        server.handle_server_event(ServerEvent::ClientDisconnected { client_id: 91 });
        assert_runtime_sizes(&server, cols.max(120), rows.max(40));
        if cols >= 120 && rows >= 40 {
            for (id, sequence) in &before {
                assert_eq!(
                    server.app.terminal_runtimes.get(id).unwrap().content_seq(),
                    *sequence
                );
            }
        }
        let sequences: Vec<_> = before
            .iter()
            .map(|(id, _)| {
                (
                    id.clone(),
                    server.app.terminal_runtimes.get(id).unwrap().content_seq(),
                )
            })
            .collect();
        assert!(!server.resize_tabs_for_only_shell_client(false));
        for (id, sequence) in sequences {
            assert_eq!(
                server.app.terminal_runtimes.get(&id).unwrap().content_seq(),
                sequence
            );
        }
        shutdown_test_runtimes(&mut server);
    }
}

#[cfg(unix)]
#[tokio::test]
async fn handoff_gap_events_do_not_resize_wide_retained_terminals() {
    let mut server = geometry_server(120, 40);
    let (control, _render) = connect_test_shell(&mut server, 92, 160, 50);
    let _ = control.recv().unwrap();
    let dying_pane = server.app.state.workspaces[0].tabs[1].root_pane;
    let dying_terminal = server.app.state.workspaces[0]
        .terminal_id(dying_pane)
        .unwrap()
        .clone();
    let mut retained = Vec::new();
    for id in server.app.state.terminals.keys() {
        let runtime = server.app.terminal_runtimes.get(id).unwrap();
        // Transferred terminals can retain known cell pixels. Clearing those
        // would also count as an unwanted resize, even with unchanged columns.
        runtime.resize(50, 160, 8, 16);
        if id != &dying_terminal {
            retained.push((id.clone(), runtime.content_seq()));
        }
    }
    server.disconnect_all_clients_for_handoff();
    assert!(server.clients.is_empty());
    assert!(!server.resize_tabs_for_only_shell_client(false));
    server.handle_internal_event_with_forwarding(AppEvent::PaneDied {
        pane_id: dying_pane,
        exit_reason: crate::platform::ChildExitReason::Exited,
    });
    assert!(server.app.find_pane(dying_pane).is_none());
    let (respond_to, response_rx) = std::sync::mpsc::channel();
    server.handle_api_request_with_shutdown_check(api::ApiRequestMessage {
        request: api::schema::Request {
            id: "handoff-gap-tab".into(),
            method: api::schema::Method::TabCreate(api::schema::TabCreateParams {
                workspace_id: Some(server.app.public_workspace_id(0)),
                cwd: None,
                focus: false,
                label: None,
                env: HashMap::new(),
            }),
        },
        respond_to,
        response_write_complete: None,
        stream_active: None,
    });
    let response: serde_json::Value = serde_json::from_str(&response_rx.recv().unwrap()).unwrap();
    assert!(response.get("error").is_none(), "{response}");
    for (id, sequence) in retained {
        let runtime = server.app.terminal_runtimes.get(&id).unwrap();
        assert_eq!(runtime.current_size(), (50, 160));
        assert_eq!(
            runtime.content_seq(),
            sequence,
            "retained terminal was resized during the handoff gap"
        );
    }
    let tab = server.app.state.workspaces[0].tabs.last().unwrap();
    let runtime = server
        .app
        .terminal_runtimes
        .get(tab.terminal_id(tab.root_pane).unwrap())
        .unwrap();
    assert_eq!(runtime.current_size(), (40, 120));
    assert!(!server.resize_tabs_for_only_shell_client(false));
    server.app.state.assert_invariants_for_test();
    shutdown_test_runtimes(&mut server);
}

#[tokio::test]
async fn headless_popup_growth_preserves_wide_size_and_direct_attach_lock() {
    let mut server = geometry_server(120, 40);
    assert!(server.resize_tabs_for_only_shell_client(false));
    let id = crate::terminal::TerminalId::alloc();
    server.app.terminal_runtimes.insert(
        id.clone(),
        crate::terminal::TerminalRuntime::test_with_screen_bytes(200, 80, b""),
    );
    server.app.state.popup_pane = Some(crate::app::state::PopupPaneState {
        pane_id: crate::layout::PaneId::alloc(),
        terminal_id: id.clone(),
        width: None,
        height: None,
    });
    let sequence = server.app.terminal_runtimes.get(&id).unwrap().content_seq();
    assert!(!server.resize_tabs_for_only_shell_client(false));
    assert_eq!(
        server.app.terminal_runtimes.get(&id).unwrap().content_seq(),
        sequence
    );
    server.app.terminal_runtimes.insert(
        id.clone(),
        crate::terminal::TerminalRuntime::test_with_screen_bytes(20, 8, b""),
    );
    server
        .app
        .state
        .direct_attach_resize_locks
        .insert(id.clone());
    assert!(!server.resize_tabs_for_only_shell_client(false));
    assert_eq!(
        server
            .app
            .terminal_runtimes
            .get(&id)
            .unwrap()
            .current_size(),
        (8, 20)
    );
    server.app.state.direct_attach_resize_locks.remove(&id);
    assert!(server.resize_tabs_for_only_shell_client(false));
    let minimum = crate::popup_size::resolve_popup_geometry(None, None, Rect::new(0, 0, 120, 40))
        .unwrap()
        .inner;
    assert_eq!(
        server
            .app
            .terminal_runtimes
            .get(&id)
            .unwrap()
            .current_size(),
        (minimum.height, minimum.width)
    );
    assert!(!server.resize_tabs_for_only_shell_client(false));
    shutdown_test_runtimes(&mut server);
}
