path=`readlink -f "${BASH_SOURCE:-$0}"`
DIR_PATH=`dirname $path`

curl -X DELETE -H "Content-Type: application/json"  http://127.0.0.1:8080/item/PEP

echo ""
echo ""

JSON_PATH="@$DIR_PATH/report.json"
curl -X POST -H "Content-Type: application/json" -d $JSON_PATH http://127.0.0.1:8080/create_initial_report

echo ""
echo ""

JSON_ADD_PATH="@$DIR_PATH/report_add.json"
curl -X POST -H "Content-Type: application/json" -d $JSON_ADD_PATH http://127.0.0.1:8080/add_report/PEP