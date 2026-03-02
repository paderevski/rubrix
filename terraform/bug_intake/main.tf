locals {
  lambda_name      = "${var.name_prefix}-bug-intake"
  table_name       = "${var.name_prefix}-bug-reports"
  api_name         = "${var.name_prefix}-bug-api"
  route_path       = "bug-report"
  lambda_src_file  = abspath("${path.module}/../../lambda_functions/bug_intake/lambda_handler.py")
  lambda_zip_path  = "${path.module}/build/bug_intake_lambda.zip"
}

resource "aws_dynamodb_table" "bug_reports" {
  name         = local.table_name
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "report_id"

  attribute {
    name = "report_id"
    type = "S"
  }

  ttl {
    attribute_name = "ttl"
    enabled        = true
  }

  tags = {
    Service = "bug-intake"
    App     = var.name_prefix
  }
}

data "archive_file" "lambda_zip" {
  type        = "zip"
  source_file = local.lambda_src_file
  output_path = local.lambda_zip_path
}

resource "aws_iam_role" "lambda_exec" {
  name = "${local.lambda_name}-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "lambda_basic" {
  role       = aws_iam_role.lambda_exec.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy" "lambda_dynamo" {
  name = "${local.lambda_name}-dynamo-policy"
  role = aws_iam_role.lambda_exec.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:GetItem"
        ]
        Resource = aws_dynamodb_table.bug_reports.arn
      }
    ]
  })
}

resource "aws_lambda_function" "bug_intake" {
  function_name    = local.lambda_name
  role             = aws_iam_role.lambda_exec.arn
  runtime          = "python3.11"
  handler          = "lambda_handler.lambda_handler"
  filename         = data.archive_file.lambda_zip.output_path
  source_code_hash = data.archive_file.lambda_zip.output_base64sha256
  timeout          = 30
  memory_size      = 256

  environment {
    variables = {
      GITHUB_OWNER        = var.github_owner
      GITHUB_REPO         = var.github_repo
      GITHUB_TOKEN        = var.github_token
      DEFAULT_LABELS      = var.default_labels
      BUG_REPORTS_TABLE   = aws_dynamodb_table.bug_reports.name
      BUG_REPORTS_TTL_DAYS = tostring(var.bug_reports_ttl_days)
      INGEST_API_KEY      = var.ingest_api_key
      RAW_REPORTS_S3_BUCKET = var.raw_reports_s3_bucket
      RAW_REPORTS_S3_PREFIX = var.raw_reports_s3_prefix
    }
  }

  depends_on = [
    aws_iam_role_policy_attachment.lambda_basic,
    aws_iam_role_policy.lambda_dynamo
  ]

  tags = {
    Service = "bug-intake"
    App     = var.name_prefix
  }
}

resource "aws_apigatewayv2_api" "bug_api" {
  name          = local.api_name
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_integration" "bug_lambda" {
  api_id                 = aws_apigatewayv2_api.bug_api.id
  integration_type       = "AWS_PROXY"
  integration_method     = "POST"
  integration_uri        = aws_lambda_function.bug_intake.invoke_arn
  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "bug_route" {
  api_id    = aws_apigatewayv2_api.bug_api.id
  route_key = "POST /${local.route_path}"
  target    = "integrations/${aws_apigatewayv2_integration.bug_lambda.id}"
}

resource "aws_apigatewayv2_stage" "default" {
  api_id      = aws_apigatewayv2_api.bug_api.id
  name        = "$default"
  auto_deploy = true
}

resource "aws_lambda_permission" "allow_api_gateway" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.bug_intake.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.bug_api.execution_arn}/*/*"
}
