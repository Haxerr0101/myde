use smithay::{
    delegate_compositor,
    delegate_shm,
    delegate_xdg_shell,

    backend::renderer::utils::on_commit_buffer_handler,

    input::{
        pointer::CursorImageStatus,
        Seat,
        SeatHandler,
        SeatState,
    },

    reexports::wayland_server::{
        backend::ClientData,
        protocol::{
            wl_seat,
            wl_surface::WlSurface,
        },
        Client,
        DisplayHandle,
    },

    utils::Serial,

    wayland::{
        compositor::{
            CompositorClientState,
            CompositorHandler,
            CompositorState,
        },
        shell::xdg::{
            PopupSurface,
            PositionerState,
            ToplevelSurface,
            XdgShellHandler,
            XdgShellState,
        },
        shm::{
            ShmHandler,
            ShmState,
        },
    },
};

pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {}

pub struct MyCompositor {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub seat_state: SeatState<MyCompositor>,
    pub shm_state: ShmState,
}

impl MyCompositor {
    pub fn new(
        display_handle: &DisplayHandle,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let compositor_state =
            CompositorState::new::<Self>(
                display_handle,
                display_handle.clone(),
            );

        let xdg_shell_state =
            XdgShellState::new::<Self>(display_handle);

        let seat_state =
            SeatState::new();

        let shm_state =
            ShmState::new::<Self>(
                display_handle,
                vec![],
            );

        Ok(Self {
            compositor_state,
            xdg_shell_state,
            seat_state,
            shm_state,
        })
    }
}

impl CompositorHandler for MyCompositor {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a Client,
    ) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .expect("ClientState missing")
            .compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
    }
}

impl XdgShellHandler for MyCompositor {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(
        &mut self,
        surface: ToplevelSurface,
    ) {
        println!("MyDE: new toplevel surface.");

        surface.send_configure();
    }

    fn new_popup(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
    ) {
        println!("MyDE: new popup surface.");
    }

    fn grab(
        &mut self,
        _surface: PopupSurface,
        _seat: wl_seat::WlSeat,
        _serial: Serial,
    ) {
        println!("MyDE: popup grab.");
    }

    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
        println!("MyDE: popup reposition request.");
    }

    fn toplevel_destroyed(
        &mut self,
        _surface: ToplevelSurface,
    ) {
        println!("MyDE: toplevel destroyed.");
    }
}

impl SeatHandler for MyCompositor {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(
        &mut self,
        _seat: &Seat<Self>,
        _focused: Option<&WlSurface>,
    ) {
    }

    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: CursorImageStatus,
    ) {
    }
}

impl ShmHandler for MyCompositor {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

delegate_compositor!(MyCompositor);
delegate_shm!(MyCompositor);
delegate_xdg_shell!(MyCompositor);
