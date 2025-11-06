use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use horrorshow::{helper::doctype, html, Raw};

use crate::mutation_tool::{Mutation, MutationResult};

fn read_file_content(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

pub fn build_html_page(data: &Vec<Mutation>, path: &Path) {
    // Group the mutations by file name
    let mut file_mutations = HashMap::new();
    for mutation in data {
        let file_name = mutation.file_name.clone();
        let file_mutations = file_mutations.entry(file_name).or_insert(Vec::new());
        file_mutations.push(mutation);
    }

    // Read file contents for each file
    let mut file_contents = HashMap::new();
    for file_name in file_mutations.keys() {
        if let Ok(content) = read_file_content(file_name) {
            file_contents.insert(file_name.clone(), content);
        }
    }

    // Calculate overall statistics
    let total_mutations: usize = file_mutations.values().map(|fm| fm.len()).sum();
    let total_killed: usize = file_mutations
        .values()
        .map(|fm| {
            fm.iter()
                .filter(|m| m.result == MutationResult::Killed)
                .count()
        })
        .sum();
    let total_survived: usize = file_mutations
        .values()
        .map(|fm| {
            fm.iter()
                .filter(|m| m.result == MutationResult::Survived)
                .count()
        })
        .sum();
    let overall_score = if (total_killed + total_survived) > 0 {
        (total_killed as f32 / (total_killed + total_survived) as f32) * 100.0
    } else {
        0.0
    };

    // Calculate per-file statistics
    let mut file_stats = Vec::new();
    for (file_name, fm) in &file_mutations {
        let killed_count = fm
            .iter()
            .filter(|m| m.result == MutationResult::Killed)
            .count();
        let survived_count = fm
            .iter()
            .filter(|m| m.result == MutationResult::Survived)
            .count();
        let score = if (killed_count + survived_count) > 0 {
            (killed_count as f32 / (killed_count + survived_count) as f32) * 100.0
        } else {
            0.0
        };
        let score_class = if score >= 90.0 {
            "score-excellent"
        } else if score >= 75.0 {
            "score-good"
        } else if score >= 50.0 {
            "score-fair"
        } else {
            "score-poor"
        };

        file_stats.push((
            file_name,
            fm.len(),
            killed_count,
            survived_count,
            score,
            score_class,
        ));
    }

    let report = format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title: "Mutant Kraken Results";
                    meta(charset="UTF-8");
                    meta(name="viewport", content="width=device-width, initial-scale=1.0");
                    style(type="text/css") {
                        : "
                        * {
                            margin: 0;
                            padding: 0;
                            box-sizing: border-box;
                        }
                        
                        body {
                            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                            min-height: 100vh;
                            padding: 20px;
                            color: #333;
                        }
                        
                        .container {
                            max-width: 1200px;
                            margin: 0 auto;
                            background: white;
                            border-radius: 20px;
                            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
                            overflow: hidden;
                        }
                        
                        .header {
                            background: linear-gradient(135deg, #2d3748 0%, #4a5568 100%);
                            color: white;
                            padding: 40px;
                            text-align: center;
                        }
                        
                        .header h1 {
                            font-size: 2.5rem;
                            font-weight: 700;
                            margin-bottom: 10px;
                            text-shadow: 0 2px 4px rgba(0,0,0,0.3);
                        }
                        
                        .header .subtitle {
                            font-size: 1.1rem;
                            opacity: 0.9;
                        }
                        
                        .stats-grid {
                            display: grid;
                            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                            gap: 20px;
                            padding: 40px;
                            background: #f8fafc;
                        }
                        
                        .stat-card {
                            background: white;
                            padding: 25px;
                            border-radius: 15px;
                            text-align: center;
                            box-shadow: 0 5px 15px rgba(0,0,0,0.08);
                            transition: transform 0.2s ease;
                        }
                        
                        .stat-card:hover {
                            transform: translateY(-2px);
                        }
                        
                        .stat-number {
                            font-size: 2.5rem;
                            font-weight: 700;
                            margin-bottom: 5px;
                        }
                        
                        .stat-label {
                            color: #64748b;
                            font-weight: 500;
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                            font-size: 0.9rem;
                        }
                        
                        .killed { color: #059669; }
                        .survived { color: #dc2626; }
                        .total { color: #3b82f6; }
                        .score { color: #7c3aed; }
                        
                        .results-section {
                            padding: 40px;
                        }
                        
                        .section-title {
                            font-size: 1.8rem;
                            font-weight: 600;
                            margin-bottom: 30px;
                            color: #1f2937;
                        }
                        
                        .file-card {
                            background: white;
                            border-radius: 12px;
                            margin-bottom: 20px;
                            box-shadow: 0 4px 12px rgba(0,0,0,0.05);
                            border: 1px solid #e5e7eb;
                            overflow: hidden;
                            transition: all 0.2s ease;
                        }
                        
                        .file-card.clickable {
                            cursor: pointer;
                        }
                        
                        .file-card:hover {
                            box-shadow: 0 8px 25px rgba(0,0,0,0.1);
                            transform: translateY(-1px);
                        }
                        
                        .file-header {
                            padding: 20px 25px;
                            background: #f9fafb;
                            border-bottom: 1px solid #e5e7eb;
                        }
                        
                        .file-name {
                            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
                            font-size: 1rem;
                            font-weight: 600;
                            color: #374151;
                            word-break: break-all;
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                        }
                        
                        .expand-indicator {
                            font-size: 0.8rem;
                            color: #6b7280;
                            transition: transform 0.2s ease;
                        }
                        
                        .file-card.expanded .expand-indicator {
                            transform: rotate(180deg);
                        }
                        
                        .mutations-detail {
                            display: none;
                            background: #f8fafc;
                            border-top: 1px solid #e5e7eb;
                        }
                        
                        .mutations-detail.visible {
                            display: block;
                        }
                        
                        .mutations-header {
                            padding: 20px 25px 10px;
                            border-bottom: 1px solid #e5e7eb;
                        }
                        
                        .mutations-header h3 {
                            font-size: 1.2rem;
                            color: #374151;
                            margin: 0;
                        }
                        
                        .mutations-list {
                            max-height: 400px;
                            overflow-y: auto;
                            padding: 10px;
                        }
                        
                        .mutation-item {
                            background: white;
                            border-radius: 8px;
                            margin-bottom: 10px;
                            padding: 15px;
                            border-left: 4px solid #e5e7eb;
                            box-shadow: 0 2px 4px rgba(0,0,0,0.05);
                            cursor: pointer;
                            transition: all 0.2s ease;
                        }
                        
                        .mutation-item:hover {
                            box-shadow: 0 4px 8px rgba(0,0,0,0.1);
                            transform: translateY(-1px);
                        }
                        
                        .mutation-item.mutation-killed {
                            border-left-color: #10b981;
                        }
                        
                        .mutation-item.mutation-survived {
                            border-left-color: #ef4444;
                        }
                        
                        .mutation-item.mutation-build-failed {
                            border-left-color: #f59e0b;
                        }
                        
                        .mutation-item.mutation-timeout {
                            border-left-color: #8b5cf6;
                        }
                        
                        .mutation-item.mutation-failed {
                            border-left-color: #6b7280;
                        }
                        
                        .mutation-header {
                            display: flex;
                            align-items: center;
                            gap: 15px;
                            margin-bottom: 10px;
                            flex-wrap: wrap;
                        }
                        
                        .mutation-line {
                            font-weight: 600;
                            color: #374151;
                            font-size: 0.9rem;
                        }
                        
                        .mutation-operator {
                            background: #e5e7eb;
                            color: #4b5563;
                            padding: 2px 8px;
                            border-radius: 12px;
                            font-size: 0.8rem;
                            font-weight: 500;
                        }
                        
                        .mutation-status {
                            padding: 3px 8px;
                            border-radius: 12px;
                            font-size: 0.75rem;
                            font-weight: 600;
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                        }
                        
                        .status-killed {
                            background: #d1fae5;
                            color: #065f46;
                        }
                        
                        .status-survived {
                            background: #fee2e2;
                            color: #991b1b;
                        }
                        
                        .status-build-failed {
                            background: #fef3c7;
                            color: #92400e;
                        }
                        
                        .status-timeout {
                            background: #ede9fe;
                            color: #6b21a8;
                        }
                        
                        .status-failed, .status-in-progress {
                            background: #f3f4f6;
                            color: #4b5563;
                        }
                        
                        .mutation-change {
                            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
                            font-size: 0.85rem;
                        }
                        
                        .change-old, .change-new {
                            margin-bottom: 5px;
                        }
                        
                        .change-label {
                            font-weight: 600;
                            color: #6b7280;
                            font-size: 0.8rem;
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                        }
                        
                        .change-old code {
                            background: #fee2e2;
                            color: #991b1b;
                            padding: 2px 6px;
                            border-radius: 4px;
                        }
                        
                        .change-new code {
                            background: #d1fae5;
                            color: #065f46;
                            padding: 2px 6px;
                            border-radius: 4px;
                        }
                        
                        /* Code Viewer Modal Styles */
                        .code-viewer-modal {
                            display: none;
                            position: fixed;
                            z-index: 1000;
                            left: 0;
                            top: 0;
                            width: 100%;
                            height: 100%;
                            background-color: rgba(0,0,0,0.5);
                            backdrop-filter: blur(5px);
                        }
                        
                        .code-viewer-modal.visible {
                            display: flex;
                            align-items: center;
                            justify-content: center;
                        }
                        
                        .code-viewer-content {
                            background: white;
                            border-radius: 12px;
                            width: 90%;
                            max-width: 1200px;
                            max-height: 90%;
                            overflow: hidden;
                            box-shadow: 0 25px 50px rgba(0,0,0,0.25);
                            display: flex;
                            flex-direction: column;
                        }
                        
                        .code-viewer-header {
                            padding: 20px 25px;
                            border-bottom: 1px solid #e5e7eb;
                            background: #f9fafb;
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                        }
                        
                        .code-viewer-title {
                            font-size: 1.2rem;
                            font-weight: 600;
                            color: #374151;
                            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
                        }
                        
                        .code-viewer-close {
                            background: none;
                            border: none;
                            font-size: 24px;
                            cursor: pointer;
                            color: #6b7280;
                            padding: 5px;
                            border-radius: 4px;
                            transition: all 0.2s ease;
                        }
                        
                        .code-viewer-close:hover {
                            background: #e5e7eb;
                            color: #374151;
                        }
                        
                        .code-viewer-body {
                            padding: 0;
                            overflow: auto;
                            flex: 1;
                        }
                        
                        .code-comparison {
                            display: grid;
                            grid-template-columns: 1fr 1fr;
                            height: 100%;
                            min-height: 400px;
                        }
                        
                        .code-panel {
                            display: flex;
                            flex-direction: column;
                        }
                        
                        .code-panel-header {
                            padding: 15px 20px;
                            font-weight: 600;
                            font-size: 0.9rem;
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                            border-bottom: 1px solid #e5e7eb;
                        }
                        
                        .code-panel-header.original {
                            background: #fef2f2;
                            color: #991b1b;
                        }
                        
                        .code-panel-header.mutated {
                            background: #f0fdf4;
                            color: #166534;
                        }
                        
                        .code-content {
                            padding: 0;
                            flex: 1;
                            overflow: auto;
                            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
                            font-size: 0.85rem;
                            line-height: 1.6;
                        }
                        
                        .code-line {
                            display: flex;
                            align-items: flex-start;
                            min-height: 1.6em;
                            padding: 0 20px;
                            border-left: 3px solid transparent;
                        }
                        
                        .code-line:hover {
                            background: #f8fafc;
                        }
                        
                        .line-number {
                            color: #9ca3af;
                            margin-right: 20px;
                            width: 40px;
                            text-align: right;
                            user-select: none;
                            flex-shrink: 0;
                        }
                        
                        .line-content {
                            flex: 1;
                            white-space: pre;
                            color: #374151;
                        }
                        
                        .code-line.highlighted {
                            background: #fef3c7;
                            border-left-color: #f59e0b;
                        }
                        
                        .code-line.highlighted .line-number {
                            color: #92400e;
                            font-weight: 600;
                        }
                        
                        .code-line.original-highlight {
                            background: #fee2e2;
                            border-left-color: #ef4444;
                        }
                        
                        .code-line.mutated-highlight {
                            background: #dcfce7;
                            border-left-color: #22c55e;
                        }
                        
                        .mutation-info {
                            padding: 20px 25px;
                            background: #f8fafc;
                            border-bottom: 1px solid #e5e7eb;
                            display: flex;
                            align-items: center;
                            gap: 15px;
                            flex-wrap: wrap;
                        }
                        
                        .mutation-info-item {
                            display: flex;
                            align-items: center;
                            gap: 8px;
                            font-size: 0.9rem;
                        }
                        
                        .mutation-info-label {
                            font-weight: 600;
                            color: #6b7280;
                        }
                        
                        .mutation-info-value {
                            color: #374151;
                        }
                        
                        @media (max-width: 768px) {
                            .code-comparison {
                                grid-template-columns: 1fr;
                                grid-template-rows: 1fr 1fr;
                            }
                            
                            .code-viewer-content {
                                width: 95%;
                                max-height: 95%;
                            }
                        }
                        
                        .file-stats {
                            display: grid;
                            grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
                            gap: 20px;
                            padding: 25px;
                        }
                        
                        .file-stat {
                            text-align: center;
                        }
                        
                        .file-stat-number {
                            font-size: 1.5rem;
                            font-weight: 700;
                            margin-bottom: 5px;
                        }
                        
                        .file-stat-label {
                            font-size: 0.8rem;
                            color: #6b7280;
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                        }
                        
                        .progress-bar {
                            width: 100%;
                            height: 8px;
                            background: #e5e7eb;
                            border-radius: 4px;
                            overflow: hidden;
                            margin: 15px 0;
                        }
                        
                        .progress-fill {
                            height: 100%;
                            background: linear-gradient(90deg, #10b981, #059669);
                            border-radius: 4px;
                            transition: width 0.5s ease;
                        }
                        
                        .score-badge {
                            display: inline-block;
                            padding: 6px 12px;
                            border-radius: 20px;
                            font-weight: 600;
                            font-size: 0.9rem;
                        }
                        
                        .score-excellent { background: #d1fae5; color: #065f46; }
                        .score-good { background: #dbeafe; color: #1e40af; }
                        .score-fair { background: #fef3c7; color: #92400e; }
                        .score-poor { background: #fee2e2; color: #991b1b; }
                        
                        .footer {
                            text-align: center;
                            padding: 30px;
                            color: #6b7280;
                            border-top: 1px solid #e5e7eb;
                            background: #f9fafb;
                        }
                        
                        @media (max-width: 768px) {
                            .header h1 { font-size: 2rem; }
                            .stats-grid { grid-template-columns: repeat(2, 1fr); padding: 20px; }
                            .results-section { padding: 20px; }
                            .file-stats { grid-template-columns: repeat(2, 1fr); }
                        }
                        ";
                    }
                }
                body {
                    div(class="container") {
                        div(class="header") {
                            h1 { : "🦑 Mutant Kraken Results"; }
                            div(class="subtitle") { : "Mutation Testing Report"; }
                        }

                        div(class="stats-grid") {
                            div(class="stat-card") {
                                div(class="stat-number total") { : format!("{}", total_mutations); }
                                div(class="stat-label") { : "Total Mutations"; }
                            }
                            div(class="stat-card") {
                                div(class="stat-number killed") { : format!("{}", total_killed); }
                                div(class="stat-label") { : "Killed"; }
                            }
                            div(class="stat-card") {
                                div(class="stat-number survived") { : format!("{}", total_survived); }
                                div(class="stat-label") { : "Survived"; }
                            }
                            div(class="stat-card") {
                                div(class="stat-number score") { : format!("{:.1}%", overall_score); }
                                div(class="stat-label") { : "Overall Score"; }
                            }
                        }

                        div(class="results-section") {
                            h2(class="section-title") { : "File Results"; }
                            @for (file_name, total_mutations, killed_count, survived_count, score, score_class) in file_stats.iter() {
                                div(class="file-card clickable", data-file-id=format!("{}", file_name.replace("'", "\\'").replace("/", "_").replace(".", "_").replace(":", "_"))) {
                                    div(class="file-header") {
                                        div(class="file-name") {
                                            : format!("{}", file_name);
                                            span(class="expand-indicator") { : "▼"; }
                                        }
                                        div(class="progress-bar") {
                                            div(class="progress-fill", style=format!("width: {}%", score));
                                        }
                                        span(class=format!("score-badge {}", score_class)) {
                                            : format!("{:.1}% Score", score);
                                        }
                                    }
                                    div(class="file-stats") {
                                        div(class="file-stat") {
                                            div(class="file-stat-number total") { : format!("{}", total_mutations); }
                                            div(class="file-stat-label") { : "Mutations"; }
                                        }
                                        div(class="file-stat") {
                                            div(class="file-stat-number killed") { : format!("{}", killed_count); }
                                            div(class="file-stat-label") { : "Killed"; }
                                        }
                                        div(class="file-stat") {
                                            div(class="file-stat-number survived") { : format!("{}", survived_count); }
                                            div(class="file-stat-label") { : "Survived"; }
                                        }
                                    }
                                    div(class="mutations-detail", id=format!("mutations-{}", file_name.replace("'", "\\'").replace("/", "_").replace(".", "_").replace(":", "_"))) {
                                        div(class="mutations-header") {
                                            h3 { : "Mutations Detail"; }
                                        }
                                        div(class="mutations-list") {
                                            @for mutation in file_mutations.get(*file_name).unwrap_or(&Vec::new()).iter() {
                                                div(class=format!("mutation-item {} mutation-clickable", match mutation.result {
                                                    MutationResult::Killed => "mutation-killed",
                                                    MutationResult::Survived => "mutation-survived",
                                                    MutationResult::BuildFailed => "mutation-build-failed",
                                                    MutationResult::Timeout => "mutation-timeout",
                                                    MutationResult::Failed => "mutation-failed",
                                                    _ => "mutation-in-progress"
                                                }),
                                                data-mutation-id=format!("{}", mutation.id),
                                                data-file-name=format!("{}", file_name),
                                                data-line-number=format!("{}", mutation.line_number),
                                                data-old-op=format!("{}", mutation.old_op),
                                                data-new-op=format!("{}", mutation.new_op),
                                                data-mutation-type=format!("{:?}", mutation.mutation_type)) {
                                                    div(class="mutation-header") {
                                                        span(class="mutation-line") { : format!("Line {}", mutation.line_number); }
                                                        span(class="mutation-operator") { : format!("{:?}", mutation.mutation_type); }
                                                        span(class=format!("mutation-status status-{}", match mutation.result {
                                                            MutationResult::Killed => "killed",
                                                            MutationResult::Survived => "survived",
                                                            MutationResult::BuildFailed => "build-failed",
                                                            MutationResult::Timeout => "timeout",
                                                            MutationResult::Failed => "failed",
                                                            _ => "in-progress"
                                                        })) { : format!("{}", mutation.result); }
                                                    }
                                                    div(class="mutation-change") {
                                                        div(class="change-old") {
                                                            span(class="change-label") { : "Original: "; }
                                                            code { : format!("{}", mutation.old_op); }
                                                        }
                                                        div(class="change-new") {
                                                            span(class="change-label") { : "Mutated: "; }
                                                            code { : format!("{}", mutation.new_op); }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div(class="footer") {
                            p { : format!("Generated by Mutant Kraken • {} files analyzed", file_mutations.len()); }
                        }
                    }

                    // Code Viewer Modal
                    div(class="code-viewer-modal", id="codeViewerModal") {
                        div(class="code-viewer-content") {
                            div(class="code-viewer-header") {
                                div(class="code-viewer-title", id="codeViewerTitle") { : "File Viewer"; }
                                button(class="code-viewer-close", onclick="closeCodeViewer()") { : "×"; }
                            }
                            div(class="mutation-info", id="mutationInfo") {
                                // Mutation info will be populated by JavaScript
                            }
                            div(class="code-viewer-body") {
                                div(class="code-comparison") {
                                    div(class="code-panel") {
                                        div(class="code-panel-header original") { : "Original Code"; }
                                        div(class="code-content", id="originalCode") {
                                            // Original code will be populated by JavaScript
                                        }
                                    }
                                    div(class="code-panel") {
                                        div(class="code-panel-header mutated") { : "Mutated Code"; }
                                        div(class="code-content", id="mutatedCode") {
                                            // Mutated code will be populated by JavaScript
                                        }
                                    }
                                }
                            }
                        }
                    }

                    script {
                        : Raw(&format!("
                        // File contents embedded in the HTML
                        const fileContents = {};", 
                        serde_json::to_string(&file_contents).unwrap_or_else(|_| "{}".to_string())
                        ));
                        : Raw("
                        
                        function getFileContent(fileName, lineNumber, oldOp, newOp) {
                            const lines = fileContents[fileName] || [];
                            
                            if (lines.length === 0) {
                                return {
                                    original: ['// File content not available'],
                                    mutated: ['// File content not available'], 
                                    targetLine: 1
                                };
                            }
                            
                            // Create mutated version by replacing the operator at the specified line
                            if (lineNumber > 0 && lineNumber <= lines.length) {
                                const originalLine = lines[lineNumber - 1];
                                const mutatedLine = originalLine.replace(oldOp, newOp);
                                return {
                                    original: lines,
                                    mutated: lines.map((line, index) => 
                                        index === lineNumber - 1 ? mutatedLine : line
                                    ),
                                    targetLine: lineNumber
                                };
                            }
                            
                            return { original: lines, mutated: lines, targetLine: lineNumber };
                        }
                        
                        function showCodeViewer(fileName, lineNumber, oldOp, newOp, mutationType) {
                            console.log('Showing code viewer for:', fileName, lineNumber, oldOp, newOp);
                            
                            const modal = document.getElementById('codeViewerModal');
                            const title = document.getElementById('codeViewerTitle');
                            const mutationInfo = document.getElementById('mutationInfo');
                            const originalCode = document.getElementById('originalCode');
                            const mutatedCode = document.getElementById('mutatedCode');
                            
                            // Update title
                            title.textContent = fileName;
                            
                            // Update mutation info
                            mutationInfo.innerHTML = `
                                <div class='mutation-info-item'>
                                    <span class='mutation-info-label'>Line:</span>
                                    <span class='mutation-info-value'>${lineNumber}</span>
                                </div>
                                <div class='mutation-info-item'>
                                    <span class='mutation-info-label'>Type:</span>
                                    <span class='mutation-info-value'>${mutationType}</span>
                                </div>
                                <div class='mutation-info-item'>
                                    <span class='mutation-info-label'>Change:</span>
                                    <span class='mutation-info-value'><code style='background: #fee2e2; color: #991b1b; padding: 2px 4px; border-radius: 3px;'>${oldOp}</code> → <code style='background: #d1fae5; color: #065f46; padding: 2px 4px; border-radius: 3px;'>${newOp}</code></span>
                                </div>
                            `;
                            
                            // Generate file content
                            const fileContent = getFileContent(fileName, parseInt(lineNumber), oldOp, newOp);
                            
                            // Render original code
                            originalCode.innerHTML = fileContent.original.map((line, index) => {
                                const lineNum = index + 1;
                                const isHighlighted = lineNum === fileContent.targetLine;
                                return `
                                    <div class='code-line ${isHighlighted ? 'original-highlight' : ''}'>
                                        <span class='line-number'>${lineNum}</span>
                                        <span class='line-content'>${line}</span>
                                    </div>
                                `;
                            }).join('');
                            
                            // Render mutated code
                            mutatedCode.innerHTML = fileContent.mutated.map((line, index) => {
                                const lineNum = index + 1;
                                const isHighlighted = lineNum === fileContent.targetLine;
                                return `
                                    <div class='code-line ${isHighlighted ? 'mutated-highlight' : ''}'>
                                        <span class='line-number'>${lineNum}</span>
                                        <span class='line-content'>${line}</span>
                                    </div>
                                `;
                            }).join('');
                            
                            // Show modal
                            modal.classList.add('visible');
                            
                            // Scroll to the highlighted line
                            setTimeout(() => {
                                const highlightedOriginal = originalCode.querySelector('.original-highlight');
                                const highlightedMutated = mutatedCode.querySelector('.mutated-highlight');
                                if (highlightedOriginal) {
                                    highlightedOriginal.scrollIntoView({ behavior: 'smooth', block: 'center' });
                                }
                                if (highlightedMutated) {
                                    highlightedMutated.scrollIntoView({ behavior: 'smooth', block: 'center' });
                                }
                            }, 100);
                        }
                        
                        function closeCodeViewer() {
                            const modal = document.getElementById('codeViewerModal');
                            modal.classList.remove('visible');
                        }
                        
                        function toggleMutations(fileId) {
                            console.log('Toggling mutations for:', fileId);
                            const mutationsDetail = document.getElementById('mutations-' + fileId);
                            const fileCard = mutationsDetail.closest('.file-card');
                            
                            if (mutationsDetail.classList.contains('visible')) {
                                mutationsDetail.classList.remove('visible');
                                fileCard.classList.remove('expanded');
                                console.log('Collapsed');
                            } else {
                                // Hide all other expanded sections
                                document.querySelectorAll('.mutations-detail.visible').forEach(detail => {
                                    detail.classList.remove('visible');
                                    detail.closest('.file-card').classList.remove('expanded');
                                });
                                
                                mutationsDetail.classList.add('visible');
                                fileCard.classList.add('expanded');
                                console.log('Expanded');
                            }
                        }
                        
                        // Add click event listeners
                        document.addEventListener('DOMContentLoaded', function() {
                            console.log('DOM loaded, setting up click handlers');
                            
                            // File card click handlers
                            document.querySelectorAll('.file-card.clickable').forEach(card => {
                                card.addEventListener('click', function(e) {
                                    e.preventDefault();
                                    const fileId = this.getAttribute('data-file-id');
                                    console.log('Card clicked, fileId:', fileId);
                                    if (fileId) {
                                        toggleMutations(fileId);
                                    }
                                });
                            });
                            
                            // Mutation item click handlers
                            document.querySelectorAll('.mutation-clickable').forEach(item => {
                                item.addEventListener('click', function(e) {
                                    e.stopPropagation(); // Prevent file card click
                                    
                                    const fileName = this.getAttribute('data-file-name');
                                    const lineNumber = this.getAttribute('data-line-number');
                                    const oldOp = this.getAttribute('data-old-op');
                                    const newOp = this.getAttribute('data-new-op');
                                    const mutationType = this.getAttribute('data-mutation-type');
                                    
                                    showCodeViewer(fileName, lineNumber, oldOp, newOp, mutationType);
                                });
                            });
                            
                            // Close modal when clicking outside
                            document.getElementById('codeViewerModal').addEventListener('click', function(e) {
                                if (e.target === this) {
                                    closeCodeViewer();
                                }
                            });
                            
                            // Close modal with Escape key
                            document.addEventListener('keydown', function(e) {
                                if (e.key === 'Escape') {
                                    closeCodeViewer();
                                }
                            });
                        });
                        ");
                    }
                }
            }
        }
    );
    // write file to Mutant-kraken-dist/report.html
    let file_path = path.join("report.html");
    let mut file = File::create(file_path).expect("Could not create report.html file");
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

        // Create mutant-kraken-dist directory
        std::fs::create_dir_all("mutant-kraken-dist").unwrap();

        // Create test data
        let mut file_mutations = HashMap::new();
        for mutation in &mutations {
            let file_name = mutation.file_name.clone();
            let file_mutations = file_mutations.entry(file_name).or_insert(Vec::new());
            file_mutations.push(mutation.clone());
        }

        // Call the function
        build_html_page(&mutations, Path::new("./mutant-kraken-dist"));

        // Read the generated HTML file
        let file_path = Path::new("mutant-kraken-dist").join("report.html");
        let mut file_content = String::new();
        File::open(file_path)
            .expect("Failed to open the generated HTML file")
            .read_to_string(&mut file_content)
            .expect("Failed to read the generated HTML file");

        // Verify that HTML content contains the new modern structure
        assert_contains(&file_content, "🦑 Mutant Kraken Results");
        assert_contains(&file_content, "Mutation Testing Report");
        assert_contains(&file_content, "Total Mutations");
        assert_contains(&file_content, "Killed");
        assert_contains(&file_content, "Survived");
        assert_contains(&file_content, "Overall Score");
        assert_contains(&file_content, "File Results");

        // Check that we have the modern card structure
        assert_contains(&file_content, r#"class="container""#);
        assert_contains(&file_content, r#"class="header""#);
        assert_contains(&file_content, r#"class="stats-grid""#);
        assert_contains(&file_content, r#"file-card clickable"#); // Updated to match new structure
        assert_contains(&file_content, r#"class="progress-bar""#);
        assert_contains(&file_content, r#"score-badge"#); // Just check for score-badge anywhere
        assert_contains(&file_content, r#"mutations-detail"#); // Check for mutations detail section
        assert_contains(&file_content, r#"expand-indicator"#); // Check for expand indicator

        for (file_name, fm) in file_mutations.iter() {
            // Check that file names are displayed
            assert_contains(&file_content, file_name);

            // Check that mutation counts are displayed
            let total_count = fm.len().to_string();
            assert_contains(&file_content, &total_count);

            let killed_count = fm
                .iter()
                .filter(|m| m.result == MutationResult::Killed)
                .count();
            let survived_count = fm
                .iter()
                .filter(|m| m.result == MutationResult::Survived)
                .count();

            assert_contains(&file_content, &killed_count.to_string());
            assert_contains(&file_content, &survived_count.to_string());
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
