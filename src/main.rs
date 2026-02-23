//
// Changed to read mega first and hd child
//
use std::path::{Path};
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::env;
// use std::process::Command as stdCommand;
use std::time::Instant as timeInstant;
use chrono::Local;
use qndr;

mod testparms;
use testparms::testparms;

fn main()  {
    let mut vecexcludef: Vec<String> = Vec::new();
    let mut vecexcluded: Vec<String> = Vec::new();
    let mut linenumx: u64 = 0;

    let args: Vec<_> = env::args().collect();
    
    let (mut bolok, parm1dir, parm2dir, parm3dir, linestrtnum, megarows_num, hdrows_num) =
       testparms(args);
    if bolok {
        let filey = File::open(parm1dir.clone()).unwrap();
        let mut readery = BufReader::new(filey);
        let mut linemega = String::new();
        let mut linenumy: u64 = 0;                            
        loop {
            match readery.read_line(&mut linemega) {
                Ok(bytes_read) => {
                   if bytes_read == 0 {
                       println!("bytes_read == 0 for {}", parm1dir);
                       bolok = false;
                       break;
                   }
                   linenumy = linenumy + 1;
                   if linenumy == 1 {
                       if linemega.trim().to_string() == "filename|filesize|filedate|dirname|refname|md5sum|locations|notes".to_string() {
                           println!("megaSVPD file is ok with size of {} rows", megarows_num);
                           break;
                       } else {
                           println!("first line of hd file is not valid: {}", linemega);
                           bolok = false;
                           break;
                       }
                   }         
                }
                Err(err) => {  
                   println!("error of {} reading {}", err, parm1dir);
                   bolok = false;
                   break;
                }
            };
        }
    }
    if bolok {
        let file = File::open(parm2dir.clone()).unwrap();
        let mut reader = BufReader::new(file);
        let mut linehd = String::new();
        let mut linenum: u64 = 0;
        loop {
            match reader.read_line(&mut linehd) {
                Ok(bytes_read) => {
                   if bytes_read == 0 {
                       println!("bytes_read == 0 for {}", parm2dir);
                       bolok = false;
                       break;
                   }
                   linenum = linenum + 1;
                   if linenum == 1 {
                       if linehd.trim().to_string() == "filename|filesize|filedate|dirname|refname|md5sum|locations|notes".to_string() {
                           println!("hd file is ok with size of {} rows", hdrows_num);
                           break;
                       } else {
                           println!("first line of hd file is not valid: {}", linehd);
                           bolok = false;
                           break;
                       }
                   } else {
                       println!("linenum after 1 for {}", parm2dir);
                       break;
                   }
                }
                Err(err) => {  
                   println!("error of {} reading {}", err, parm2dir);
                   bolok = false;
                   break;
                }
            };
        }
    }
    if bolok {
        let filey = File::open(parm3dir.clone()).unwrap();
        let mut readery = BufReader::new(filey);
        let mut lineex = String::new();
        let mut linenumy: u64 = 0;
        loop {
            match readery.read_line(&mut lineex) {
                Ok(bytes_read) => {
                   if bytes_read == 0 {
                       println!("exclude file is has no records");
                       bolok = false;
                       break;
                   }
                   linenumy = linenumy + 1;
                   if linenumy == 1 {
                       if lineex.trim().to_string() == "exclude file".to_string() {
                           println!("exclude file is ok");
                       } else {
                           println!("first line of exclude file is not valid: {}", lineex);
                           bolok = false;
                       }
                   } else {
                       break;
                   }
                }
                Err(err) => {  
                   println!("error of {} reading {}", err, parm3dir);
                   bolok = false;
                   break;
                }
            };
        }
    }
    
    if bolok {
        let fileex = File::open(parm3dir.clone()).unwrap();
        let mut readerex = BufReader::new(fileex);
        let mut lineex = String::new();
        let mut lineexnum: u64 = 0;
        loop {
              match readerex.read_line(&mut lineex) {
                 Ok(bytes_read) => {
                     if bytes_read == 0 {
                         break;
                     }
                     lineexnum = lineexnum + 1;
                     if lineexnum > 1 {
                         let excl: String = lineex.trim().to_string();
                         if excl.len() < 3 {
                             println!("exclusion less than 3 characters: {}", excl);
                             bolok = false;
                             break;
                         } else {
                             let exclv: String = excl[2..].to_string();
//                             println!("exclusion value:-{}-", exclv);
                             if excl[..2].to_string() == "d ".to_string() {
                                 vecexcluded.push(exclv);
                             } else if excl[..2].to_string() == "f ".to_string() {
                                 vecexcludef.push(exclv);
                             } else {
                                 println!("exclusion invalid first two characters {}", excl);
                                 bolok = false;
                                 break;
                             }
                         }   
                     }
                     lineex.clear();
                 }
                 Err(err) => {
                     println!("error {} when reading exclusion file", err);
                     bolok = false;   
                     break;
                 }
              };
        }
        if lineexnum < 2 {
            println!("exclusion file {} has no records", parm3dir);
            bolok = false;
        } else {
            lineexnum = lineexnum - 1;
            println!("exclusion file {} has {} records", parm3dir, lineexnum);
        }
    }
    if bolok {
        let mut outseq: u32 = 1;
        let mut more1out: String = format!("./more1{:02}.excout", outseq);
        let mut just1out: String = format!("./just1{:02}.neout", outseq);
        let mut size0out: String = format!("./size0{:02}.neout", outseq);
        let mut excludout: String = format!("./excluded{:02}.excout", outseq);
        let mut nomegaupout: String = format!("./nomegaup{:02}.neout", outseq);
        let mut errout: String = format!("./generrors{:02}.errout", outseq);
        loop {
               if !Path::new(&errout).exists() && !Path::new(&more1out).exists()
                  && !Path::new(&just1out).exists() && !Path::new(&size0out).exists()
                  && !Path::new(&excludout).exists() && !Path::new(&nomegaupout).exists() {
                   break;
               } else {
                   outseq = outseq + 1;
                   more1out = format!("./more1{:02}.excout", outseq);
                   just1out = format!("./just1{:02}.neout", outseq);
                   size0out = format!("./size0{:02}.neout", outseq);
                   excludout = format!("./excluded{:02}.excout", outseq);
                   nomegaupout = format!("./nomegaup{:02}.neout", outseq);
                   errout = format!("./generrors{:02}.errout", outseq);
               }
        }          
        let mut excludefile = File::create(excludout).unwrap();
        let mut nomegaupfile = File::create(nomegaupout).unwrap();
        let mut more1file = File::create(more1out).unwrap();
        let mut just1file = File::create(just1out).unwrap();
        let mut size0file = File::create(size0out).unwrap();
        let mut errfile = File::create(errout).unwrap();
        let filehd = File::open(parm2dir.clone()).unwrap();
        let mut readerhd = BufReader::new(filehd);
        let filemega = File::open(parm1dir.clone()).unwrap();
        let mut readermega = BufReader::new(filemega);
        let mut linehd = String::new();
        let mut linehdfmt = String::new();
        let mut linemega = String::new();
        let mut linestatnum: u64 = 0;
        let mut vecmegasavefiles: Vec<String> = Vec::new();
        let mut megamd5curr = String::new();
        let mut megafilecurr = String::new();
        let mut megamd5save: String = "none".to_string();
        let mut hdfilemd5: String = "none".to_string();
        let mut inptfilenm = String::new();
        let mut inptfilelen: i64 = 0;
        let mut bolrdhd = true;
        let mut bolhdend = false;
        let start_time = timeInstant::now();
        let mut linenummega: u64 = 0; 
        let mut errcount: u64 = 0;
// loop thru major mega file
        loop {
              if !bolok {
                  break;
              }
              if bolhdend {
                  break;
              }
              match readermega.read_line(&mut linemega) {
                 Ok(bytes_read) => {
                     if bytes_read == 0 {
                         println!("{} files end of megaSVPD list", linenummega);
                         break;
                     }
                     linenummega = linenummega + 1;
                     if linenummega > 1 {
                         let veclinea: Vec<&str> = linemega.split("|").collect();
                         if veclinea.len() < 6 {
                             let stroutput = format!("invalid mega record {} line {}", linemega, linenummega);
                             writeln!(&mut errfile, "{}", stroutput).unwrap();
                         } else {
                             let mut megamd5a: String = veclinea[5].to_string();
                             if megamd5a.len() > 32 {
                                 if megamd5a[..1].to_string() == '"'.to_string() {
                                     megamd5a = megamd5a[1..33].to_string();
                                 } else {
                                     megamd5a = megamd5a[..32].to_string();
                                 }
                             }
                             let mut megafilenma: String = veclinea[0].to_string();
                             if megafilenma[..1].to_string() == '"'.to_string() {
                                 megafilenma = megafilenma[1..(megafilenma.len()-1)].to_string();
                             }
                             if megafilenma.contains("&apos;") {
                                 megafilenma = megafilenma.replace("&apos;", "'");
                             }
                             if megafilenma.contains("&amp;") {
                                 megafilenma = megafilenma.replace("&amp;", "&");
                             }
                             if !qndr::alphanumeric_with_symbols(&megafilenma,".-_ []()^$#,&'%!~@{}+=–’;:`") {
                                 writeln!(&mut errfile, "mega not alphanumeric {} | {} | {}", megafilenma, veclinea[0], linenummega).unwrap();
                                 errcount = errcount + 1;
                                 if errcount > 1500 {
                                     writeln!(&mut errfile, "errors exceed 1500 exiting at line {}", linenumx).unwrap();
                                     bolok = false;
                                 }
                             }
                             if linenummega == 2 {
                                 megamd5save = megamd5a;
                                 vecmegasavefiles.push(megafilenma);
                             } else {
                                 if megamd5save == megamd5a {
                                     vecmegasavefiles.push(megafilenma);
                                 } else {
                                     megamd5curr = megamd5a;
                                     megafilecurr = megafilenma;
// loop thru minor hd file
                                     loop {
                                           if !bolok {
                                               break;
                                           }
                                           if bolrdhd {
                                               match readerhd.read_line(&mut linehd) {
                                                  Ok(bytes_read) => {
                                                      bolrdhd = false;
                                                      if bytes_read == 0 {
                                                          let diffy = start_time.elapsed();
                                                          let minsy: f64 = diffy.as_secs() as f64/60 as f64;
                                                          let dateyy = Local::now();
                                                          println!("line number {} records elapsed time {:.1} mins at {} completed", linenumx, minsy, dateyy.format("%H:%M:%S"));
                                                          bolhdend = true;
                                                          break;
                                                      }
                                                      linestatnum = linestatnum + 1;
                                                      linenumx = linenumx + 1;
                                                      if linenumx <= linestrtnum {
                                                          bolrdhd = true;
                                                      } else {
                                                          if linestatnum > 50000 {
                                                              let diffy = start_time.elapsed();
                                                              let minsy: f64 = diffy.as_secs() as f64/60 as f64;
                                                              let dateyy = Local::now();
                                                              println!("line number {} records elapsed time {:.1} mins at {}", linenumx, minsy, dateyy.format("%H:%M:%S"));
                                                              linestatnum = 0;
                                                          }
                                                          let vecline: Vec<&str> = linehd.split("|").collect();
                                                          hdfilemd5 = vecline[5].to_string();
                                                          if hdfilemd5.len() > 32 {
                                                              hdfilemd5 = hdfilemd5[..32].to_string();
                                                          }
                                                          linehdfmt = format!("{}|{}|{}|{}|{}|{}|{}", vecline[4], vecline[0], vecline[3], vecline[1], vecline[2], hdfilemd5, linenumx);
                                                          let inptdir = vecline[3].to_string();
                                                          inptfilenm = vecline[0].to_string();
                                                          let mut bolex = false;
                                                          for strexclf in &vecexcludef {
                                                               if inptfilenm.contains(strexclf) {
                                                                   bolex = true;
                                                                   writeln!(&mut excludefile, "f {}", linehdfmt).unwrap();
                                                                   break;
                                                               }
                                                          }
                                                          if !bolex {
                                                              for strexcld in &vecexcluded {
                                                                   if inptdir.contains(strexcld) {
                                                                       bolex = true;
                                                                       writeln!(&mut excludefile, "d {}", linehdfmt).unwrap();
                                                                       break;
                                                                   }
                                                              }
                                                          }
                                                          if bolex {
                                                              bolrdhd = true;
                                                          } else {
                                                              let nlenhd: i64 = vecline[1].parse().unwrap_or(-9999);
                                                              if nlenhd < 0 {
                                                                  writeln!(&mut errfile, "hd invalid length {} | {}", inptfilenm, linenumx).unwrap();
                                                                  bolok = false;
                                                                  break;
                                                              } else {
                                                                  inptfilelen = nlenhd;
                                                              }
                                                              if inptfilenm[..1].to_string() == '"'.to_string() {
                                                                  inptfilenm = inptfilenm[1..(inptfilenm.len()-1)].to_string();
                                                              }
                                                              if !qndr::alphanumeric_with_symbols(&inptfilenm,".-_ []()^$#,&'%!~@{}+=–’;:`") {
                                                                  writeln!(&mut errfile, "hd not alphanumeric {} | {}", inptfilenm, linenumx).unwrap();
                                                                  errcount = errcount + 1;
                                                                  if errcount > 1500 {
                                                                      writeln!(&mut errfile, "errors exceed 1500 exiting at line {}", linenumx).unwrap();
                                                                      bolok = false;
                                                                  }
                                                              }
                                                          }
                                                      }
                                                      linehd.clear()
                                                  }
                                                  Err(err) => {  
                                                      println!("error of {} reading {}", err, parm2dir);
                                                      bolok = false;
                                                      break;
                                                  }
                                               }
                                           }
                                           if !bolrdhd {
                                               if hdfilemd5 > megamd5save {
                                                   if inptfilelen < 1 {
                                                       writeln!(&mut size0file, "1-{} | nomegaup", linehdfmt).unwrap();
                                                   } else {
                                                       writeln!(&mut nomegaupfile, "1-{}", linehdfmt).unwrap();
                                                   }
                                                   bolrdhd = true;
                                               } else if hdfilemd5 == megamd5save {
                                                   bolrdhd = true;
                                                   let mut nummatch = 0;
                                                   for mega in &vecmegasavefiles {
                                                        if mega == &inptfilenm {
                                                            nummatch = nummatch + 1;
                                                        }
                                                   }
                                                   if nummatch < 1 {
                                                       if inptfilelen < 1 {
                                                           writeln!(&mut size0file, "1-{} | nomegaup", linehdfmt).unwrap();
                                                       } else {
                                                           writeln!(&mut nomegaupfile, "2-{}", linehdfmt).unwrap();
                                                       }
                                                   } else if nummatch < 2 {
                                                       if inptfilelen < 1 {
                                                           writeln!(&mut size0file, "1-{} | just1", linehdfmt).unwrap();
                                                       } else {
                                                           writeln!(&mut just1file, "2-{}", linehdfmt).unwrap();
                                                       }
                                                   } else {
                                                       if inptfilelen < 1 {
                                                           writeln!(&mut size0file, "1-{} | more1", linehdfmt).unwrap();
                                                       } else {
                                                           writeln!(&mut more1file, "2-{}", linehdfmt).unwrap();
                                                       }
                                                   }
                                               } else {
                                                   vecmegasavefiles.clear();
                                                   megamd5save = megamd5curr.clone();
                                                   vecmegasavefiles.push(megafilecurr.clone());
                                                   break;
                                               }
                                           }
                                     }
  // end loop of minor hd file
                                 }
                             }
                         }
                     }
                     linemega.clear();
                 }
                 Err(err) => {
                     let stroutput = format!("error of {} reading {}", err, parm1dir);
                     println!("{}", stroutput);
                     writeln!(&mut errfile, "{}", stroutput).unwrap();
                     break;
                 }
              };
        }
// end of loop of major mega file
        if bolok {
            if !bolhdend {
// loop thru minor hd file
                loop {
                    if !bolok {
                        break;
                    }
                    if bolrdhd {
                        match readerhd.read_line(&mut linehd) {
                            Ok(bytes_read) => {
                               bolrdhd = false;
                               if bytes_read == 0 {
                                   let diffy = start_time.elapsed();
                                   let minsy: f64 = diffy.as_secs() as f64/60 as f64;
                                   let dateyy = Local::now();
                                   println!("line number {} records elapsed time {:.1} mins at {} completed", linenumx, minsy, dateyy.format("%H:%M:%S"));
                                   break;
                               }
                               linestatnum = linestatnum + 1;
                               linenumx = linenumx + 1;
                               if linenumx <= linestrtnum {
                                   bolrdhd = true;
                               } else {
                                   if linestatnum > 50000 {
                                       let diffy = start_time.elapsed();
                                       let minsy: f64 = diffy.as_secs() as f64/60 as f64;
                                       let dateyy = Local::now();
                                       println!("line number {} records elapsed time {:.1} mins at {}", linenumx, minsy, dateyy.format("%H:%M:%S"));
                                       linestatnum = 0;
                                   }
                                   let vecline: Vec<&str> = linehd.split("|").collect();
                                   hdfilemd5 = vecline[5].to_string();
                                   if hdfilemd5.len() > 32 {
                                       hdfilemd5 = hdfilemd5[..32].to_string();
                                   }
                                   linehdfmt = format!("{}|{}|{}|{}|{}|{}|{}", vecline[4], vecline[0], vecline[3], vecline[1], vecline[2], hdfilemd5, linenumx);
                                   let inptdir = vecline[3].to_string();
                                   inptfilenm = vecline[0].to_string();
                                   let mut bolex = false;
                                   for strexclf in &vecexcludef {
                                        if inptfilenm.contains(strexclf) {
                                            bolex = true;
                                            writeln!(&mut excludefile, "f {}", linehdfmt).unwrap();
                                            break;
                                        }
                                   }
                                   if !bolex {
                                       for strexcld in &vecexcluded {
                                            if inptdir.contains(strexcld) {
                                                bolex = true;
                                                writeln!(&mut excludefile, "d {}", linehdfmt).unwrap();
                                                break;
                                            }
                                       }
                                   }
                                   if bolex {
                                       bolrdhd = true;
                                   } else {
                                       let nlenhd: i64 = vecline[1].parse().unwrap_or(-9999);
                                       if nlenhd < 0 {
                                           writeln!(&mut errfile, "hd invalid length {} | {}", inptfilenm, linenumx).unwrap();
                                           bolok = false;
                                           break;
                                       } else {
                                           inptfilelen = nlenhd;
                                       }
                                       if inptfilenm[..1].to_string() == '"'.to_string() {
                                           inptfilenm = inptfilenm[1..(inptfilenm.len()-1)].to_string();
                                       }
                                       if !qndr::alphanumeric_with_symbols(&inptfilenm,".-_ []()^$#,&'%!~@{}+=–’;:`") {
                                           writeln!(&mut errfile, "hd not alphanumeric {} | {}", inptfilenm, linenumx).unwrap();
                                           errcount = errcount + 1;
                                           if errcount > 1500 {
                                               writeln!(&mut errfile, "errors exceed 1500 exiting at line {}", linenumx).unwrap();
                                               bolok = false;
                                           }
                                       }
                                   }
                               }
                               linehd.clear()
                            }
                            Err(err) => {  
                               println!("error of {} reading {}", err, parm2dir);
                               bolok = false;
                               break;
                            }
                        }
                    }
                    if !bolrdhd {
                        bolrdhd = true;
                        if hdfilemd5 > megamd5save {
                            if inptfilelen < 1 {
                                writeln!(&mut size0file, "1-{} | nomegaup", linehdfmt).unwrap();
                            } else {
                                writeln!(&mut nomegaupfile, "1-{}", linehdfmt).unwrap();
                            }
                        } else if hdfilemd5 == megamd5save {
                            let mut nummatch = 0;
                            for mega in &vecmegasavefiles {
                                 if mega == &inptfilenm {
                                     nummatch = nummatch + 1;
                                 }
                            }
                            if nummatch < 1 {
                                if inptfilelen < 1 {
                                    writeln!(&mut size0file, "1-{} | nomegaup", linehdfmt).unwrap();
                                } else {
                                    writeln!(&mut nomegaupfile, "2-{}", linehdfmt).unwrap();
                                }
                            } else if nummatch < 2 {
                                if inptfilelen < 1 {
                                    writeln!(&mut size0file, "1-{} | just1", linehdfmt).unwrap();
                                } else {
                                    writeln!(&mut just1file, "2-{}", linehdfmt).unwrap();
                                }
                            } else {
                                if inptfilelen < 1 {
                                    writeln!(&mut size0file, "1-{} | more1", linehdfmt).unwrap();
                                } else {
                                    writeln!(&mut more1file, "2-{}", linehdfmt).unwrap();
                                }
                            }
                        } else {
                            if hdfilemd5 > megamd5curr {
                                if inptfilelen < 1 {
                                    writeln!(&mut size0file, "1-{} | nomegaup", linehdfmt).unwrap();
                                } else {
                                    writeln!(&mut nomegaupfile, "1-{}", linehdfmt).unwrap();
                                }
                            } else if hdfilemd5 == megamd5curr {
                                if inptfilenm == megafilecurr {
                                    if inptfilelen < 1 {
                                        writeln!(&mut size0file, "1-{} | just1", linehdfmt).unwrap();
                                    } else {
                                        writeln!(&mut just1file, "2-{}", linehdfmt).unwrap();
                                    }
                                } else {
                                    if inptfilelen < 1 {
                                        writeln!(&mut size0file, "1-{} | nomegaup", linehdfmt).unwrap();
                                    } else {
                                        writeln!(&mut nomegaupfile, "1-{}", linehdfmt).unwrap();
                                    }
                                }
                            } else {
                                if inptfilelen < 1 {
                                    writeln!(&mut size0file, "1-{} | nomegaup", linehdfmt).unwrap();
                                } else {
                                    writeln!(&mut nomegaupfile, "1-{}", linehdfmt).unwrap();
                                }
                            }
                        }
                    }
                }
  // end loop of minor hd file
            }
        }    
    }
    if linenumx < 1 {
        linenumx = 1;
    }
    if bolok {
         println!("{} files successfully completed", (linenumx-1));
    } else {
         println!("{} files but errors. Check error file.", (linenumx-1));
    }
}
