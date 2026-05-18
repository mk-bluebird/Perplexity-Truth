//! Evidence-gap visualization utilities.
//!
//! Produces simple ASCII or SVG representations of which evidence elements
//! (documents, witnesses, logs, declassified records) are present or missing,
//! without sending any analytics or external data.

use std::fmt::Write as _;

#[derive(Debug, Clone)]
pub struct EvidenceGapRow {
    pub label: String,
    pub has_document: bool,
    pub has_witness: bool,
    pub has_logs: bool,
    pub has_declassified: bool,
}

/// Render a simple ASCII matrix representing evidence presence / gaps.
pub fn render_ascii_matrix(rows: &[EvidenceGapRow]) -> String {
    let mut out = String::new();
    // Header
    let _ = writeln!(
        out,
        "{:<20} | {:^10} | {:^10} | {:^10} | {:^14}",
        "Element", "Docs", "Witness", "Logs", "Declassified"
    );
    let _ = writeln!(out, "{}", "-".repeat(20 + 3 + 10 + 3 + 10 + 3 + 10 + 3 + 14));
    // Rows
    for row in rows {
        let _ = writeln!(
            out,
            "{:<20} | {:^10} | {:^10} | {:^10} | {:^14}",
            row.label,
            if row.has_document { "✔" } else { "✘" },
            if row.has_witness { "✔" } else { "✘" },
            if row.has_logs { "✔" } else { "✘" },
            if row.has_declassified { "✔" } else { "✘" },
        );
    }
    out
}

/// Render a minimal, self-contained SVG heatmap-like grid.
/// No network calls, no analytics.
pub fn render_svg_matrix(rows: &[EvidenceGapRow]) -> String {
    let cell_size = 24;
    let width = 5 * cell_size + 200; // label + 4 columns
    let height = (rows.len() as i32 + 1) * cell_size;

    let mut svg = String::new();
    let _ = writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">"#,
        width = width,
        height = height
    );

    // Column labels
    let labels = ["Docs", "Witness", "Logs", "Declass"];
    for (i, label) in labels.iter().enumerate() {
        let x = 200 + (i as i32) * cell_size;
        let y = cell_size - 6;
        let _ = writeln!(
            svg,
            r#"<text x="{x}" y="{y}" font-size="10" text-anchor="middle">{label}</text>"#,
            x = x + cell_size / 2,
            y = y,
            label = label
        );
    }

    // Rows
    for (row_idx, row) in rows.iter().enumerate() {
        let y = ((row_idx + 1) as i32) * cell_size;

        // Row label
        let _ = writeln!(
            svg,
            r#"<text x="5" y="{ty}" font-size="10">{label}</text>"#,
            ty = y + (cell_size / 2),
            label = row.label
        );

        let flags = [
            row.has_document,
            row.has_witness,
            row.has_logs,
            row.has_declassified,
        ];

        for (col_idx, has_flag) in flags.iter().enumerate() {
            let x = 200 + (col_idx as i32) * cell_size;
            let fill = if *has_flag { "#4caf50" } else { "#f44336" };
            let _ = writeln!(
                svg,
                r#"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="{fill}" stroke="#000" />"#,
                x = x,
                y = y,
                size = cell_size - 4,
                fill = fill
            );
        }
    }

    let _ = writeln!(svg, "</svg>");
    svg
}
