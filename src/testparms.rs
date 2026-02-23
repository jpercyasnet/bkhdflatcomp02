use std::path::{Path};
use std::process::Command as stdCommand;
pub fn testparms<T: std::fmt::Display + std::convert::AsRef<std::ffi::OsStr>> (args: Vec<T>) -> (bool, String, String, String, u64, u64, u64) {
    let mut bolok: bool = true;
    let mut parm1dir = String::new();
    let mut parm2dir = String::new();
    let mut parm3dir = String::new();
    let mut linestrtnum: u64 = 1;
    let mut bkrows_num: u64 = 0;
    let mut hdrows_num: u64 = 0;
    if args.len() < 2 {
        println!(" no input parameters");
        bolok = false;
    } else {
        println!("The first argument is {}", args[1]);
        if args.len() < 3 {
            println!("The Only first argument and no other arguments");
            bolok = false;
        } else {
            println!("The second argument is {}", args[2]);
            if args.len() < 4 {
                println!("The Only first and second arguments and no other arguments");
                bolok = false;
            } else {
                println!("The third argument is {}", args[3]);
                if Path::new(&args[1]).exists() {
                    println!("The first argument {} exists", args[1]);
                    parm1dir = args[1].to_string();                    
                    let outputy = stdCommand::new("wc")
                         .arg("-l")
                         .arg(&parm1dir)
                         .output()
                         .expect("failed to execute process");
                    let strouty = String::from_utf8_lossy(&outputy.stdout);
                    let vecout: Vec<&str> = strouty.split(" ").collect();
                    let numlinesy: i64 = vecout[0].parse().unwrap_or(-9999);
                    if numlinesy == -9999 {
                        println!("size of {} is invalid for wc -l command call", vecout[0]);
                        bolok = false;
                    } else {
                        bkrows_num = numlinesy as u64;
                        if bkrows_num < 10 {
                            println!("size of {} is less than 10 for {}", bkrows_num, parm1dir);
                            bolok = false;
                        }                    
                    }
                } else {
                    println!("The first argument {} does not exist", args[1]);
                    bolok = false;
                }
                if !Path::new(&args[2]).exists() {
                    println!("The second argument {} does not exist", args[2]);
                    bolok = false;
                } else {
                    println!("The second argument {} exists", args[2]);
                    parm2dir = args[2].to_string();
                    let outputx = stdCommand::new("wc")
                         .arg("-l")
                         .arg(&parm2dir)
                         .output()
                         .expect("failed to execute process");
                    let stroutx = String::from_utf8_lossy(&outputx.stdout);
                    let vecout: Vec<&str> = stroutx.split(" ").collect();
                    let numlinesx: i64 = vecout[0].parse().unwrap_or(-9999);
                    if numlinesx == -9999 {
                        println!("size of {} is invalid for wc -l command call", vecout[0]);
                        bolok = false;
                    } else {
                        hdrows_num = numlinesx as u64;
                        if hdrows_num < 2 {
                            println!("size of {} is less than 2 for {}", hdrows_num, parm2dir);
                            bolok = false;
                        }
                    }
                }
                if !Path::new(&args[3]).exists() {
                    println!("The third argument {} does not exist", args[3]);
                    bolok = false;
                } else {
                    println!("The third argument {} exists", args[3]);
                    parm3dir = args[3].to_string();
                    let outputy = stdCommand::new("wc")
                         .arg("-l")
                         .arg(&parm3dir)
                         .output()
                         .expect("failed to execute process");
                    let strouty = String::from_utf8_lossy(&outputy.stdout);
                    let vecouty: Vec<&str> = strouty.split(" ").collect();
                    let numlinesy: i64 = vecouty[0].parse().unwrap_or(-9999);
                    if numlinesy == -9999 {
                        println!("size of {} is invalid for wc -l command call", vecouty[0]);
                        bolok = false;
                    } else {
                        let exrows_num = numlinesy as u64;
                        if exrows_num < 2 {
                            println!("size of {} is less than 2 for {}", exrows_num, parm3dir);
                            bolok = false;    
                        }
                    }
                    if args.len() > 4 {
                       let arg4 = args[4].to_string();
                       let numarg4: i64 = arg4.parse().unwrap_or(-9999);
                       if numarg4 < 2 {
                           println!("argument 4 is not valid value: {}", arg4);
                           bolok = false;
                       } else {
                           println!("argument 4 is valid value: {}", arg4);
                           linestrtnum = numarg4 as u64;
                       }
                    }
                }
            }
        }
    }
    (bolok, parm1dir, parm2dir, parm3dir, linestrtnum, bkrows_num, hdrows_num)
}
