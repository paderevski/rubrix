How to build and deploy pandoc converter to AWS

# assumes you've logged in to aws

docker build --platform linux/amd64 --provenance=false -t pandoc-lambda .
docker tag pandoc-lambda:latest 706194210592.dkr.ecr.us-east-1.amazonaws.com/pandoc-lambda:latest
docker push 706194210592.dkr.ecr.us-east-1.amazonaws.com/pandoc-lambda:latest

aws ecr get-login-password | docker login --username AWS --password-stdin <registry-url>

aws lambda update-function-code \
  --function-name pandoc-converter \
  --image-uri 706194210592.dkr.ecr.us-east-1.amazonaws.com/pandoc-lambda:latest \
  --region us-east-1

# Wait for update to complete
aws lambda wait function-updated --function-name pandoc-converter --region us-east-1


# Test
curl -X POST https://aminsl4ogh.execute-api.us-east-1.amazonaws.com/convert \
  -H 'Content-Type: application/json' \
  -d '{"markdown": "# Test\n\nHello", "format": "docx"}' \
  --output test.docx

# Test with a reference/template DOCX (Pandoc --reference-doc)
# 1) Put your style template at ./reference.docx
# 2) Base64 encode it and include in payload

TEMPLATE_B64=$(base64 < reference.docx | tr -d '\n')
curl -X POST https://aminsl4ogh.execute-api.us-east-1.amazonaws.com/convert \
  -H 'Content-Type: application/json' \
  -d "{\"markdown\":\"# Test\\n\\nStyled output\",\"format\":\"docx\",\"use_reference_docx\":true,\"reference_docx_base64\":\"$TEMPLATE_B64\"}" \
  --output test-with-template.docx

## NOTE THE above CURL doesn't seem to pass the b64 file through...

# Watch CloudWatch logs (last 10 minutes)
aws logs tail /aws/lambda/pandoc-converter --since 10m --region us-east-1

# Watch only template/debug lines from this converter
aws logs tail /aws/lambda/pandoc-converter --since 10m --region us-east-1 \
  --format short | grep '\[convert\]'