use std::{collections::HashMap, fs::File, io::Write, path::Path};

use horrorshow::{helper::doctype, html};

use crate::mutation_tool::{Mutation, MutationResult};

pub fn build_html_page(data: &Vec<Mutation>) {
    // Group the mutations by file name
    let mut file_mutations = HashMap::new();
    for mutation in data {
        let file_name = mutation.file_name.clone();
        let file_mutations = file_mutations.entry(file_name).or_insert(Vec::new());
        file_mutations.push(mutation);
    }
    let report = format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title: "Kode Kraken Results";
                }
                body {
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
                            @for (file_name, fm) in file_mutations.iter() {
                                tr {
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
                                        : format!(
                                            "{}%",
                                            (fm.iter().filter(|m| m.result == MutationResult::Killed).count() as f32
                                            / (fm.len() -
                                                fm
                                                .iter()
                                                .filter(|m|
                                            m.result != MutationResult::Killed &&
                                            m.result != MutationResult::Survived)
                                        .count()) as f32) * 100.0
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    );
    // write file to kode-kraken-dist/report.html
    let file_path = Path::new("kode-kraken-dist").join("report.html");
    let mut file = File::create(file_path).unwrap();
    file.write_all(report.as_bytes()).unwrap();
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use crate::mutation_tool::{Mutation, MutationOperators, MutationResult};

    use super::build_html_page;

    #[test]
    fn test_build_html_page() {
        // Create some sample mutations
        let mutation1 = Mutation::new(
            0,
            10,
            "new_op1".to_string(),
            "old_op1".to_string(),
            1,
            MutationOperators::ArithmeticReplacementOperator,
            "file1".to_string(),
        );
        let mutation2 = Mutation::new(
            0,
            15,
            "new_op2".to_string(),
            "old_op2".to_string(),
            2,
            MutationOperators::AssignmentReplacementOperator,
            "file1".to_string(),
        );
        let mutation3 = Mutation::new(
            0,
            8,
            "new_op3".to_string(),
            "old_op3".to_string(),
            1,
            MutationOperators::ElvisLiteralChangeOperator,
            "file2".to_string(),
        );
        let mutation4 = Mutation::new(
            0,
            12,
            "new_op4".to_string(),
            "old_op4".to_string(),
            3,
            MutationOperators::LogicalReplacementOperator,
            "file2".to_string(),
        );

        let mutations = vec![
            mutation1.clone(),
            mutation2.clone(),
            mutation3.clone(),
            mutation4.clone(),
        ];

        // Create test data
        let mut file_mutations = HashMap::new();
        for mutation in &mutations {
            let file_name = mutation.file_name.clone();
            let file_mutations = file_mutations.entry(file_name).or_insert(Vec::new());
            file_mutations.push(mutation.clone());
        }

        // Call the function
        build_html_page(&mutations);

        // Read the generated HTML file
        let file_path = Path::new("kode-kraken-dist").join("report.html");
        let mut file_content = String::new();
        File::open(file_path)
            .expect("Failed to open the generated HTML file")
            .read_to_string(&mut file_content)
            .expect("Failed to read the generated HTML file");

        // Verify that HTML content contains information about each file and mutation
        assert_contains(
            &file_content,
            "<th class=\"tg-baqh\" colspan=\"5\">Kode Kraken Results</th>",
        );
        assert_contains(&file_content, "<td class=\"tg-0lax\">File Name</td>");
        assert_contains(&file_content, "<td class=\"tg-0lax\"># of Mutations</td>");
        assert_contains(&file_content, "<td class=\"tg-0lax\"># Survived</td>");
        assert_contains(&file_content, "<td class=\"tg-0lax\"># Killed</td>");
        assert_contains(&file_content, "<td class=\"tg-0lax\">Score</td>");

        for (file_name, fm) in file_mutations.iter() {
            assert_contains(
                &file_content,
                &format!("<td class=\"tg-ycr8\">{}</td>", file_name),
            );
            assert_contains(
                &file_content,
                &format!("<td class=\"tg-lax\">{}</td>", fm.len()),
            );
            assert_contains(
                &file_content,
                &format!(
                    "<td class=\"tg-lax\">{}</td>",
                    fm.iter()
                        .filter(|m| m.result == MutationResult::Survived)
                        .count()
                ),
            );
            assert_contains(
                &file_content,
                &format!(
                    "<td class=\"tg-lax\">{}</td>",
                    fm.iter()
                        .filter(|m| m.result == MutationResult::Killed)
                        .count()
                ),
            );
            let score = (fm
                .iter()
                .filter(|m| m.result == MutationResult::Killed)
                .count() as f32
                / (fm.len()
                    - fm.iter()
                        .filter(|m| {
                            m.result != MutationResult::Killed
                                && m.result != MutationResult::Survived
                        })
                        .count()) as f32)
                * 100.0;
            assert_contains(
                &file_content,
                &format!("<td class=\"tg-lax\">{:.2}%</td>", score),
            );
        }
    }

    fn assert_contains(haystack: &str, needle: &str) {
        assert!(
            haystack.contains(needle),
            "{}",
            format!("Expected content:\n{}\nTo contain:\n{}", haystack, needle)
        );
    }
}
