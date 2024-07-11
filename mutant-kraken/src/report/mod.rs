use std::{fs::File, io::Write, path::Path};

use horrorshow::{helper::doctype, html};

use crate::mutation_tool::Mutation;

pub struct Report<'a> {
    pub files: Vec<FileReport>,
    pub path: &'a Path,
    pub total_mutants: usize,
    pub total_killed: usize,
    pub total_survived: usize,
}

pub struct FileReport {
    pub file: Vec<u8>,
    pub mutations: Vec<Mutation>,
    pub file_name: String,
}

const REPORT_CSS: &str = include_str!("../../assets/mutation/mutation-report.css");

impl<'a> Report<'a> {
    pub fn new(files: Vec<FileReport>, path: &'a Path, tm: usize, tk: usize, ts: usize) -> Self {
        Report {
            files,
            path,
            total_killed: tk,
            total_mutants: tm,
            total_survived: ts,
        }
    }

    pub fn create_report(&self) {
        let report = format!(
            "{}",
            html! {
                : doctype::HTML;
                html {
                    head {
                        title: "Mutant Kraken Results";
                        link(rel="stylesheet", type="text/css", href="mutation-report.css")
                    }
                    body {
                        table {
                            tr {
                                th: "Filename";
                                th: "Total Mutations";
                                th: "Mutants Killed";
                                th: "Mutants Survived";
                                th: "Mutation Score";
                            }
                            tr {
                                @for file in self.files.iter() {
                                    td: &file.file_name;
                                    td: "0";
                                    td: "0";
                                    td: "0";
                                    td: "0";
                                }
                            }
                        }
                    }
                }
            }
        );

        // write file to Mutant-kraken-dist/report.html
        let file_path = self.path.join("report.html");
        let mut file = File::create(file_path).expect("Could not create report.html file");
        file.write_all(report.as_bytes()).unwrap();

        let css_file = self.path.join("mutation-report.css");
        let mut file = File::create(css_file).unwrap();
        file.write_all(REPORT_CSS.as_bytes()).unwrap();
    }
}
