use futures::FutureExt;
use tokio::net::TcpStream;
use tokio_postgres::tls::{NoTls, NoTlsStream};
use tokio_postgres::{Client, Config, Connection, Error, SimpleQueryMessage};

async fn connect_raw(s: &str) -> Result<(Client, Connection<TcpStream, NoTlsStream>), Error> {
    let socket = TcpStream::connect("127.0.0.1:5432").await.unwrap();
    let config = s.parse::<Config>().unwrap();
    config.connect_raw(socket, NoTls).await
}

async fn connect(s: &str) -> Client {
    let (client, connection) = connect_raw(s).await.unwrap();
    let connection = connection.map(|r| r.unwrap());
    tokio::spawn(connection);
    client
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        connect("host=127.0.0.1 port=5432 user=tmp password=tmp dbname=tmp sslmode=disable").await;

    let messages = client
        .simple_query(
            "CREATE TEMPORARY TABLE foo (
                id SERIAL,
                name TEXT
            );
            INSERT INTO foo (name) VALUES ('hello'), ('world');
            SELECT * FROM foo ORDER BY id",
        )
        .await
        .unwrap();

    match messages[0] {
        SimpleQueryMessage::CommandComplete(0) => {}
        _ => panic!("unexpected message"),
    }
    match messages[1] {
        SimpleQueryMessage::CommandComplete(2) => {}
        _ => panic!("unexpected message"),
    }
    match &messages[2] {
        SimpleQueryMessage::Row(row) => {
            assert_eq!(row.get(0), Some("1"));
            assert_eq!(row.get(1), Some("hello"));
        }
        _ => panic!("unexpected message"),
    }
    match &messages[3] {
        SimpleQueryMessage::Row(row) => {
            assert_eq!(row.get(0), Some("2"));
            assert_eq!(row.get(1), Some("world"));
        }
        _ => panic!("unexpected message"),
    }
    match messages[4] {
        SimpleQueryMessage::CommandComplete(2) => {}
        _ => panic!("unexpected message"),
    }

    Ok(())
}
