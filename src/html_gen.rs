
use std::{path::Path, fs::File, io::Write, collections::HashMap};

use horrorshow::{html, helper::doctype};

use crate::mutation::{Mutation, MutationResult};


pub fn build_html_page(_fileName: &Vec<String>,data: &Vec<Mutation>){
    // Group the mutations by file name
    let mut file_mutations = HashMap::new();
    for mutation in data {
        let file_name = mutation.file_name.clone();
        let file_mutations = file_mutations.entry(file_name).or_insert(Vec::new());
        file_mutations.push(mutation);
    }
    let report = format!("{}", html!{
        : doctype::HTML;
        html {
            head {
                title: "Hello World";
            }
            body {
                // Attributes
                style(type="text/css") {
                    : "
                    .tg  {border-collapse:collapse;border-spacing:0;}
                    .tg td{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
                    overflow:hidden;padding:10px 5px;word-break:normal;}
                    .tg th{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
                    font-weight:normal;overflow:hidden;padding:10px 5px;word-break:normal;}
                    .tg .tg-ycr8{background-color:#ffffff;text-align:left;vertical-align:top}
                    .tg .tg-baqh{text-align:center;vertical-align:top}
                    .tg .tg-0lax{text-align:left;vertical-align:top}
                    ";
                }
                table(class="tg") {
                    thead {
                        tr {
                            th(class="tg-baqh", colspan="5") {
                                : "Kode Kraken Results";
                            }
                        }
                    }
                    tbody {
                        tr {
                            td(class="tg-0lax") {
                                : "File Name";
                            }
                            td(class="tg-0lax") {
                                : "# of Mutations";
                            }
                            td(class="tg-0lax") {
                                : "# Survived";
                            }
                            td(class="tg-0lax") {
                                : "# Killed";
                            }
                            td(class="tg-0lax") {
                                : "Score";
                            }
                        }
                        tr {
                            @for (file_name, fm) in file_mutations.iter() {
                                td(class="tg-ycr8") {
                                    : format!("{}", file_name);
                                }
                                td(class="tg-lax") {
                                    : format!("{}", fm.len());
                                }
                                td(class="tg-lax") {
                                    : format!("{}", fm.iter().filter(|m| m.result == MutationResult::Survived).count());
                                }
                                td(class="tg-lax") {
                                    : format!("{}", fm.iter().filter(|m| m.result == MutationResult::Killed).count());
                                }
                                td(class="tg-lax") {
                                    : "0";
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    // write file to kode-kraken-dist/report.html
    let file_path = Path::new("kode-kraken-dist").join("report.html");
    let mut file = File::create(file_path).unwrap();
    file.write_all(report.as_bytes()).unwrap();
}
/*
<style type="text/css">
.tg  {border-collapse:collapse;border-spacing:0;}
.tg td{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
  overflow:hidden;padding:10px 5px;word-break:normal;}
.tg th{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
  font-weight:normal;overflow:hidden;padding:10px 5px;word-break:normal;}
.tg .tg-ycr8{background-color:#ffffff;text-align:left;vertical-align:top}
.tg .tg-baqh{text-align:center;vertical-align:top}
.tg .tg-0lax{text-align:left;vertical-align:top}
</style>
<table class="tg">
<thead>
  <tr>
    <th class="tg-baqh" colspan="5">Kode Kraken Results</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td class="tg-0lax">File Name</td>
    <td class="tg-0lax"># of Mutations<br></td>
    <td class="tg-0lax"># Survived</td>
    <td class="tg-0lax"># Killed</td>
    <td class="tg-0lax">Score<br></td>
  </tr>
  <tr>
    <td class="tg-ycr8">Example.txt</td>
    <td class="tg-0lax">3</td>
    <td class="tg-0lax">2</td>
    <td class="tg-0lax">1</td>
    <td class="tg-0lax">33<br></td>
  </tr>
</tbody>
</table>
 */