swagger_file='/tmp/swagger-fibermc-auto.json'
output_dir='./rust'

curl -sk https://localhost:5001/swagger/v1-pre/swagger.json > $swagger_file
npx @openapitools/openapi-generator-cli generate -i $swagger_file  -g rust -o $output_dir

