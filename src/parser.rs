use nom::{IResult, multi::separated_list1, bytes::{streaming::tag, complete::take_while1}, combinator::{value, map}, branch::alt, character::complete::multispace1, sequence::tuple};

pub fn zero_digit(input: &str) -> IResult<&str, u32> {
    value(0, tag("Нолик"))(input)
}

pub fn one_digit(input: &str) -> IResult<&str, u32> {
    value(1, tag("Целковый"))(input)
}

pub fn two_digit(input: &str) -> IResult<&str, u32> {
    value(2, tag("Полушка"))(input)
}

pub fn four_digit(input: &str) -> IResult<&str, u32> {
    value(4, tag("Четвертушка"))(input)
}

pub fn eight_digit(input: &str) -> IResult<&str, u32> {
    value(8, tag("Осьмушка"))(input)
}

pub fn sixteen_digit(input: &str) -> IResult<&str, u32> {
    value(16, tag("Пудовичок"))(input)
}

pub fn any_digit(input: &str) -> IResult<&str, u32> {
    alt((zero_digit, one_digit, two_digit, four_digit, eight_digit, sixteen_digit))(input)
}

pub fn rus_number(input: &str) -> IResult<&str, u32> {
    map(separated_list1(multispace1, any_digit), |x: Vec<u32>| x.iter().sum())(input)
}

pub fn sum(input: &str) -> IResult<&str, u32> {
    map(
        separated_list1(
            tuple((multispace1, tag("да"), multispace1)),
            rus_number
        ),
        |x: Vec<u32>| x.iter().sum::<u32>(),
    )(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|x: char| x.is_alphabetic())(input)
}

pub fn var_init(input: &str) -> IResult<&str, (&str, u32)> {
    map(
        tuple((identifier, tuple((multispace1, tag("есть"), multispace1)), sum)),
        |x| (x.0, x.2),
    )(input)

}
