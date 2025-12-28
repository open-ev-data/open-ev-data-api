module.exports = {
  branches: [
    'main'
  ],
  plugins: [
    [
      '@semantic-release/commit-analyzer',
      {
        preset: 'conventionalcommits',
        releaseRules: [
          { type: 'feat', release: 'minor' },
          { type: 'fix', release: 'patch' },
          { type: 'perf', release: 'patch' },
          { type: 'revert', release: 'patch' },
          { type: 'docs', release: false },
          { type: 'style', release: false },
          { type: 'chore', release: false },
          { type: 'refactor', release: 'patch' },
          { type: 'test', release: false },
          { type: 'build', release: false },
          { type: 'ci', release: false },
          { breaking: true, release: 'major' }
        ],
        parserOpts: {
          noteKeywords: ['BREAKING CHANGE', 'BREAKING CHANGES']
        }
      }
    ],
    [
      '@semantic-release/release-notes-generator',
      {
        preset: 'conventionalcommits',
        presetConfig: {
          types: [
            { type: 'feat', section: 'Features' },
            { type: 'fix', section: 'Bug Fixes' },
            { type: 'perf', section: 'Performance Improvements' },
            { type: 'revert', section: 'Reverts' },
            { type: 'docs', section: 'Documentation', hidden: false },
            { type: 'style', section: 'Styles', hidden: true },
            { type: 'chore', section: 'Miscellaneous Chores', hidden: false },
            { type: 'refactor', section: 'Code Refactoring' },
            { type: 'test', section: 'Tests', hidden: true },
            { type: 'build', section: 'Build System', hidden: false },
            { type: 'ci', section: 'CI', hidden: false }
          ]
        }
      }
    ],
    [
      '@semantic-release/changelog',
      {
        changelogFile: 'CHANGELOG.md',
        changelogTitle: '# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThis project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) and [Conventional Commits](https://www.conventionalcommits.org/).'
      }
    ],
    [
      '@semantic-release/exec',
      {
        prepareCmd: 'sed -i \'s/^version = ".*"/version = "${nextRelease.version}"/\' Cargo.toml && cargo generate-lockfile && chmod +x scripts/release/*.sh && ./scripts/release/prepare-artifacts.sh && ./scripts/release/generate-coverage.sh && ./scripts/release/update-coverage-badge.sh && ./scripts/release/build-linux.sh ${nextRelease.version} && ./scripts/release/build-docker.sh ${nextRelease.version}'
      }
    ],
    [
      '@semantic-release/git',
      {
        assets: [
          'CHANGELOG.md',
          'Cargo.toml',
          'Cargo.lock',
          'README.md'
        ],
        message: 'chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}'
      }
    ],
    [
      '@semantic-release/github',
      {
        successComment: false,
        labels: false,
        releasedLabels: false,
        assets: [
          {
            path: 'dist/ev-server-x86_64-unknown-linux-gnu.tar.gz',
            label: 'ev-server (Linux x86_64)'
          },
          {
            path: 'dist/ev-etl-x86_64-unknown-linux-gnu.tar.gz',
            label: 'ev-etl (Linux x86_64)'
          },
          {
            path: 'dist/coverage-report.tar.gz',
            label: 'Coverage Report (HTML)'
          },
          {
            path: 'dist/coverage-summary.json',
            label: 'Coverage Summary (JSON)'
          }
        ]
      }
    ]
  ]
};
