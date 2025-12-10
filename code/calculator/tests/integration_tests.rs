use rstest::{fixture, rstest};
use calculator::parsemath::parser::Expression;
use calculator::parsemath::visitors::{Evaluator, ExpressionVisitor, PrettyPrinterVisitor};

#[fixture]
fn expression() -> Expression {
    Expression::Multiply(
        Box::new(Expression::Grouping(Box::new(Expression::Add(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Number(2.0)),
        )))),
        Box::new(Expression::Grouping(Box::new(Expression::Subtract(
            Box::new(Expression::Number(3.0)),
            Box::new(Expression::Number(4.0)),
        )))),
    )
}

#[rstest]
fn integration_test_evaluate_expression_with_visitor(expression: Expression) {
    let mut evaluator = Evaluator {};
    let result = evaluator.visit_expression(&expression);
    assert_eq!(result.unwrap(), -3.0);
}

#[rstest]
fn integration_test_evaluate_expression_with_visitor_div_by_zero() {
    let expression = Expression::Divide(
        Box::new(Expression::Number(1.0)),
        Box::new(Expression::Number(0.0)),
    );

    let mut evaluator = Evaluator {};
    let result = evaluator.visit_expression(&expression);
    assert!(result.is_err());
}

#[rstest]
fn pretty_print_expression_with_visitor(expression: Expression) {
    let mut printer = PrettyPrinterVisitor {
        output: String::new(),
    };
    let result = printer.visit_expression(&expression);
    assert_eq!(result.unwrap(), "(1 + 2) * (3 - 4)");
}
