mod controller;
mod entry_point;
mod infrastructure;
mod init;
mod log;
mod use_case;

#[ic_cdk::init]
fn init() {
    init::init();
}
