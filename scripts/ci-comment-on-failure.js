const fs = require('fs');
const github = require('@actions/github');

// Convert ANSI codes to HTML
function ansiToHtml(text) {
    const ansiColorMap = {
        '0': 'inherit',     // reset
        '1': 'font-weight: bold',
        '30': 'color: #000000',
        '31': 'color: #ff5555',
        '32': 'color: #55ff55',
        '33': 'color: #ffff55',
        '34': 'color: #5555ff',
        '35': 'color: #ff55ff',
        '36': 'color: #55ffff',
        '37': 'color: #ffffff',
        '90': 'color: #808080',
        '91': 'color: #ff8888',
        '92': 'color: #88ff88',
        '93': 'color: #ffff88',
        '94': 'color: #8888ff',
        '95': 'color: #ff88ff',
        '96': 'color: #88ffff',
        '97': 'color: #ffffff'
    };

    let html = text
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\x1b\[([0-9;]*)m/g, (match, code) => {
            if (code === '' || code === '0') return '</span>';
            const codes = code.split(';');
            const styles = codes
                .map(c => ansiColorMap[c])
                .filter(s => s)
                .join('; ');
            return styles ? `<span style="${styles}">` : '';
        });

    return html.replace(/<\/span>/g, '</span>').replace(/span>/g, 'span>');
}

const checkOutputFiles = [
    { name: 'Clippy (no features)', file: 'clippy-no-features-output.txt' },
    { name: 'Clippy (all features)', file: 'clippy-output.txt' },
    { name: 'Documentation', file: 'docs-output.txt' },
    { name: 'Rustfmt', file: 'rustfmt-output.txt' },
    { name: 'MSRV', file: 'msrv-output.txt' },
    { name: 'Minimum Versions', file: 'minimum-versions-output.txt' },
    { name: 'Dependencies', file: 'dependancies-output.txt' }
];

let failureMessage = '❌ **CI Checks Failed**\n\n';
let hasFailures = false;

for (const check of checkOutputFiles) {
    try {
        if (fs.existsSync(check.file)) {
            const output = fs.readFileSync(check.file, 'utf8');
            if (output.trim()) {
                const htmlOutput = ansiToHtml(output);
                failureMessage += `<details>\n<summary><b>${check.name}</b></summary>\n\n<pre style="background: #1e1e1e; color: #d4d4d4; padding: 12px; border-radius: 6px; overflow-x: auto; font-family: monospace; line-height: 1.4;">${htmlOutput}</pre>\n</details>\n\n`;
                hasFailures = true;
            }
        }
    } catch (err) {
        console.log(`Could not read ${check.file}: ${err.message}`);
    }
}

if (hasFailures) {
    const context = JSON.parse(process.env.GITHUB_CONTEXT);
    const octokit = github.getOctokit(process.env.GITHUB_TOKEN);

    octokit.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.name,
        body: failureMessage
    }).then(() => {
        throw new Error('CI checks failed. See PR comment for details.');
    }).catch((error) => {
        console.error('Failed to create comment:', error);
        process.exit(1);
    });
}
