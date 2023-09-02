import json 

data = {}
ticker = ""

def get_category_for_value(value_name):
    income_statement_keys = ["revenue", 
                             "total-cogs", 
                             "operating-expense", 
                             "interest-expense", 
                             "net-income", 
                             "eps-basic", 
                             "shares-outstanding-basic"]
    
    balance_sheet_keys = ["cash-and-equivalents",
                          "total-assets",
                          "short-term-debt",
                          "long-term-debt",
                          "total-liabilities"]
    
    cash_flow_statement_keys = ["operating-cash-flow",
                                "investing-cash-flow",
                                "capital-expenditure",
                                "financing-cash-flow",
                                "dividends-paid",
                                "dividends-per-share"]
    
    financial_rations_keys = ["avg-share-price"]

    name_values_maping = {
        "income-statement":income_statement_keys,
        "balance-sheet":balance_sheet_keys,
        "cash-flow-statement":cash_flow_statement_keys,
        "financial-ratios": financial_rations_keys
    }

    for key, value in name_values_maping.items():
        if value_name in value:
            return key
        
    raise Exception("Unknown key " + value_name)
        

with open('PEP2.csv') as reader:
    header = reader.readline()
    years = header.strip("\n").split(sep=',')
    ticker = years[0]
    for year in years[1:]:
        data[year] = {}
        data[year]["income-statement"] = {}
        data[year]["balance-sheet"] = {}
        data[year]["cash-flow-statement"] = {}
        data[year]["financial-ratios"] = {}

    print(str(years))

    data_lines = reader.readlines()
    for line in data_lines:
        line_data = line.strip("\n").split(sep=',')
        value_key = line_data[0]
        for idx_year, val in enumerate(line_data[1:]):
            category = get_category_for_value(value_key)
            print("category {category} years[idx_year]={yr}".format(category=category,yr=years[idx_year+1]))
            print(str(data[years[idx_year+1]]))
            if category not in data[years[idx_year+1]]: 
                data[years[idx_year+1]][category] = {}
            data[years[idx_year+1]][category][value_key] = float(val)

json_object = json.dumps(data, indent = 4) 
# Writing to sample.json
with open(ticker + ".json", "w") as outfile:
    outfile.write(json_object)

print(json_object)
