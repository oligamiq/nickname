owner="nziq53"
repo="nickname"
file_name="test_iphone.yml"

gh api "repos/${owner}/${repo}/actions/workflows/${file_name}/runs?per_page=100" \
| jq -r '.workflow_runs[].id' \
| xargs -P4 -I{} gh api repos/{owner}/{repo}/actions/runs/{} -X DELETE

# https://qiita.com/tippy/items/79ca3f7b7bcac1d92136

# bash rm_workflow.sh
