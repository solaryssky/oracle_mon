use oracle::Connection;
use std::time::{Instant, SystemTime, Duration};
use telnet::Telnet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

fn main(){

    

    const ARR_SIZE: usize = 3;
    let arr_host: [&str; ARR_SIZE] = ["host1", "host2", "host3"];
    let arr_port: [u16; ARR_SIZE] = [1521, 1521, 1521];
    let arr_user: [&str; ARR_SIZE] = ["", "", ""];
    let arr_pass: [&str; ARR_SIZE] = ["", "", ""];
    let arr_base: [&str; ARR_SIZE] = ["", "", ""];
    let arr_table: [&str; ARR_SIZE] = ["table1", "table1", "table1"];
    let arr_sql: [&str; ARR_SIZE] = ["SELECT COUNT(ROW_ID) FROM tbl WHERE MSISDN = 79120000000", 
                                     "SELECT COUNT(ROW_ID) FROM tbl WHERE MSISDN = 79120000000",
                                     "SELECT TO_NUMBER( regexp_replace  (value , '[^[:digit:]]', '')) AS second FROM sys.V_$DATAGUARD_STATS WHERE name = 'apply lag'"];
    let arr_msg_name: [&str; ARR_SIZE] = ["count", "count", "lag"];
    


for i in 0..ARR_SIZE{

    let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let timestamp_nanos = duration_since_epoch.as_nanos(); // u128
    
    
    //telnet
    //let result_telnet = Telnet::connect((host, port), 256);
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(arr_host[i]).expect("Invalid address")), arr_port[i]);
    let result_telnet = Telnet::connect_timeout(&address, 256, Duration::from_secs(5));

    match result_telnet{
        Ok(_) => {
            println!("telnet,hostn={},hport={} telnet=1i {}", arr_host[i], arr_port[i], timestamp_nanos);
        },
        Err(_) => {
            println!("telnet,hostn={},hport={} telnet=0i {}", arr_host[i], arr_port[i], timestamp_nanos);
            return;
        }
    }
    
    
    //ping
    let addr = &arr_host[i].parse().unwrap();
    let data = [1,2,3,4,5,6,7,8];  // ping data
    let timeout = Duration::from_secs(1);
    let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
    let result_ping = ping_rs::send_ping(&addr, timeout, &data, Some(&options));
    
    //if telnet OK -> connect to BD
    
    //connect with ServiceName
    //let conn = Connection::connect(username, password, host.to_owned() + "/" + database).unwrap();
    //connect with SID

    let now = Instant::now();    
    let conn = Connection::connect(arr_user[i], arr_pass[i], "(DESCRIPTION = (ADDRESS = (PROTOCOL = TCP)(HOST = ".to_owned() + arr_host[i] + ")(PORT = " + arr_port[i].to_string().as_str() + ")) (CONNECT_DATA = (SID = " + arr_base[i] + ")))").unwrap();
    


    let mut stmt = conn.statement(arr_sql[i]).build().unwrap();
    let rows = stmt.query(&[]).unwrap();


    match result_ping {
        // Ok(reply) => println!("Reply from {}: bytes={} time={}ms TTL={}", reply.address, data.len(), reply.rtt, options.ttl),
         Ok(reply) => {
            for row_result in rows {

                // print column values
            for (_, val) in row_result.expect("error result db").sql_values().iter().enumerate() {
                  println!("querytime,user={},hostn={},bd={},tbl={} {}={}i,time={}i,ping={} {}",arr_user[i], arr_host[i], arr_base[i], arr_table[i], arr_msg_name[i], val, now.elapsed().as_millis(), reply.rtt, timestamp_nanos);
                
             }
        
            }
            

         },
         Err(e) => println!("{:?}", e)
     }




//Ok(())
    }
}
