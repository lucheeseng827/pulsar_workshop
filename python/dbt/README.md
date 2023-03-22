running the folder

install dbt
```bash
pip install dbt

```

initialize the project
```bash
dbt init my_datawarehouse_project
```
Note: : In your DBT project's dbt_project.yml file, set the profile field to the name of your datawarehouse profile:
```bash
profile: my_bigquery_profile
```


```bash
dbt run
dbt test
```
