const fs = require('fs');

module.exports = ({ github, context }) => {
    const checkOutputFiles = [
        { name: 'Rust Formating', file: 'rustfmt-output.txt' },
        { name: 'Clippy', file: 'clippy-output.txt' },
        { name: 'Documentation', file: 'docs-output.txt' },
        { name: 'MSRV', file: 'msrv-output.txt' },
        { name: 'Minimum Versions', file: 'minimum-versions-output.txt' },
        { name: 'Dependencies', file: 'dependancies-output.txt' }
    ];

    let failureMessage = `
        <h1>❌ **CI Checks Failed**</h1>
        <div>Some checks have failed. Please review the errors and fix them</div>
    `;
    let hasFailures = false;

    for (const check of checkOutputFiles) {
        try {
            if (fs.existsSync(check.file)) {
                const output = fs.readFileSync(check.file, 'utf8');
                if (output.trim()) {
                    failureMessage += `<details>\n<summary><b>${check.name}</b></summary>\n\n<pre style="background: #1e1e1e; color: #d4d4d4; padding: 12px; border-radius: 6px; overflow-x: auto; font-family: monospace; line-height: 1.4;">${output}</pre>\n</details>\n\n`;
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
}
