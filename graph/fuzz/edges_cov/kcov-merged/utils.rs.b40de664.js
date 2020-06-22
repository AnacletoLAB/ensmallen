var data = {lines:[
{"lineNum":"    1","line":"use rand::Rng;"},
{"lineNum":"    2","line":"const CHARSET: &[u8] = b\"ABCDEFGHIJKLMNOPQRSTUVWXYZ\\"},
{"lineNum":"    3","line":"                        abcdefghijklmnopqrstuvwxyz\\"},
{"lineNum":"    4","line":"                        0123456789)(*&^%$#@!~\";"},
{"lineNum":"    5","line":""},
{"lineNum":"    6","line":"pub fn random_string(len: usize) -> String{","class":"lineCov","hits":"1","order":"35",},
{"lineNum":"    7","line":"    let mut rng = rand::thread_rng();","class":"lineCov","hits":"1","order":"36",},
{"lineNum":"    8","line":""},
{"lineNum":"    9","line":"    (0..len)","class":"lineCov","hits":"1","order":"37",},
{"lineNum":"   10","line":"        .map(|_| {","class":"lineCov","hits":"1","order":"38",},
{"lineNum":"   11","line":"            let idx = rng.gen_range(0, CHARSET.len());","class":"lineCov","hits":"1","order":"42",},
{"lineNum":"   12","line":"            CHARSET[idx] as char","class":"lineCov","hits":"1","order":"43",},
{"lineNum":"   13","line":"        })","class":"lineCov","hits":"1","order":"39",},
{"lineNum":"   14","line":"        .collect()","class":"lineCov","hits":"1","order":"40",},
{"lineNum":"   15","line":"}","class":"lineCov","hits":"1","order":"41",},
]};
var percent_low = 25;var percent_high = 75;
var header = { "command" : "only_edges", "date" : "2020-06-22 09:48:42", "instrumented" : 9, "covered" : 9,};
var merged_data = [];
