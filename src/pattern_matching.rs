pub fn run() {
    // Enums look like this
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;

    fn route(ip_kind: IpAddrKind) {}
}