// コンパイラは標準でlib.rsのみを検索する
// 以下のモジュールを定義しておけばそのモジュール名に対応するファイルを作って行けば良くなる
// そういうふうにできている
//
// communicator
//   - pub client
//      pub connect()
//   - pub network
//      pub connect()
//      - pub server
//          - pub connect()
//
//   - tests
//
// lib.rs
// client.rs
// network
//  - mod.rs
//  - server.rs
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    use super::network;

    #[test]
    fn try_to_connect_by_client() {
        client::connect();
    }

    #[test]
    fn try_to_connect_by_network() {
        network::connect();
    }

    #[test]
    fn try_to_connect_by_server() {
        network::server::connect();
    }
}
