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

    pub fn parse_config() -> ModalOptions {
        todo!()
    }

    pub fn as_tuple(&self) -> (bool, bool, bool, bool) {
        (
            self.rho,
            self.sigma,
            self.tau,
            self.eta
        )
    }
}