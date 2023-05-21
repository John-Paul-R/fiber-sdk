mkdir -p ./.openapi-generator/templates/ && cd $_
curl -L https://api.github.com/repos/OpenAPITools/openapi-generator/tarball | tar xz
mv `ls`/modules/openapi-generator/src/main/resources/rust ./rust
\rm -rf OpenAPITools-openapi-generator-*
cd rust
# remove all lib implementations except reqwest, which is what we'll be using
ls -d ./*/ | grep -v reqwest | xargs rm -rf
