swagger_file='/tmp/swagger-fibermc-auto.json'
output_dir='./rust'

curl -sk https://localhost:5001/swagger/v0/swagger.json > $swagger_file
npx @openapitools/openapi-generator-cli generate -i $swagger_file  -g rust -o $output_dir \
    --package-name 'fibermc-sdk' \
#    --release-note 'extremely early alpha. quite unstable at present.' \

