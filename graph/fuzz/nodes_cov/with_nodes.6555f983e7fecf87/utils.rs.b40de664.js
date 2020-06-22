var data = {lines:[
{"lineNum":"    1","line":"use rand::Rng;"},
{"lineNum":"    2","line":"const CHARSET: &[u8] = b\"ABCDEFGHIJKLMNOPQRSTUVWXYZ\\"},
{"lineNum":"    3","line":"                        abcdefghijklmnopqrstuvwxyz\\"},
{"lineNum":"    4","line":"                        0123456789)(*&^%$#@!~\";"},
{"lineNum":"    5","line":""},
{"lineNum":"    6","line":"pub fn random_string(len: usize) -> String{","class":"lineCov","hits":"2","order":"2","possible_hits":"2",},
{"lineNum":"    7","line":"    let mut rng = rand::thread_rng();","class":"lineCov","hits":"1","order":"3","possible_hits":"1",},
{"lineNum":"    8","line":""},
{"lineNum":"    9","line":"    (0..len)","class":"lineCov","hits":"3","order":"4","possible_hits":"3",},
{"lineNum":"   10","line":"        .map(|_| {","class":"lineCov","hits":"3","order":"5","possible_hits":"3",},
{"lineNum":"   11","line":"            let idx = rng.gen_range(0, CHARSET.len());","class":"lineCov","hits":"1","order":"7","possible_hits":"1",},
{"lineNum":"   12","line":"            CHARSET[idx] as char","class":"linePartCov","hits":"1","order":"8","possible_hits":"3",},
{"lineNum":"   13","line":"        })","class":"lineCov","hits":"2","order":"6","possible_hits":"2",},
{"lineNum":"   14","line":"        .collect()","class":"lineCov","hits":"1","order":"9","possible_hits":"1",},
{"lineNum":"   15","line":"}","class":"lineCov","hits":"1","order":"10","possible_hits":"1",},
]};
var percent_low = 25;var percent_high = 75;
var header = { "command" : "with_nodes", "date" : "2020-06-22 09:16:05", "instrumented" : 9, "covered" : 9,};
var merged_data = [];
