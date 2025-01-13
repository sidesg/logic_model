pub struct ModalOptions {
    rho: bool,
    sigma: bool,
    tau: bool,
    eta: bool
}

impl ModalOptions {
    pub fn all_true() -> ModalOptions {
        ModalOptions {
            rho: true,
            sigma: true,
            tau: true,
            eta: true
        }
    }

    pub fn new_default() -> ModalOptions {
        ModalOptions {
            rho: true,
            sigma: true,
            tau: true,
            eta: false
        }       
    }

    pub fn parse_config() -> ModalOptions {
        todo!()
    }

    pub fn rho(&self) -> bool {
        self.rho
    }

    pub fn sigma(&self) -> bool {
        self.sigma
    }

    pub fn tau(&self) -> bool {
        self.tau
    }

    pub fn eta(&self) -> bool {
        self.eta
    }
}