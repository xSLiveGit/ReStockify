path=`readlink -f "${BASH_SOURCE:-$0}"`
DIR_PATH=`dirname $path`
JSON_PATH="@$DIR_PATH/report.json"
curl -X POST -H "Content-Type: application/json" -d $JSON_PATH http://127.0.0.1:8080/push_initial_report
