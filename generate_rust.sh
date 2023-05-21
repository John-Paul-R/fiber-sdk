swagger_file='/tmp/swagger-fibermc-auto.json'
templates_dir='.openapi-generator/templates/rust'
output_dir='./rust'

# verify templates compile
#(
#    cd $templates_dir && cargo check
#)

curl -sk https://localhost:5001/swagger/v0/swagger.json > $swagger_file
npx @openapitools/openapi-generator-cli generate -i $swagger_file  -g rust -o $output_dir \
    --package-name 'fibermc-sdk' \
    -t $templates_dir \
    --remove-operation-id-prefix \
#    --global-property debugModels=$debug \
#    --release-note 'extremely early alpha. quite unstable at present.' \
