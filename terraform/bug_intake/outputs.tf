output "bug_report_url" {
  description = "Set this as BUG_REPORT_URL in Catie"
  value       = "${aws_apigatewayv2_api.bug_api.api_endpoint}/bug-report"
}

output "bug_reports_table_name" {
  description = "DynamoDB table storing bug report envelopes"
  value       = aws_dynamodb_table.bug_reports.name
}

output "lambda_function_name" {
  value = aws_lambda_function.bug_intake.function_name
}
