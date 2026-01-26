#[cfg(test)]

fn scanning_template(inputs: Vec<&str>, expected: Vec<Vec<&str>>) {
    let pairs = inputs.iter().zip(expected.iter());

    for (input, expected) in pairs {
        let result = super::scan(*input);

        // Converts expected from a Vec<&&str> to a Vec<String>
        let owned_expected: Vec<String> = expected.iter()
            .map(|x| String::from(*x))
            .collect();

        assert_eq!(owned_expected, result);
    }
}

#[test]
fn scanning() {
    let inputs = vec![
        "a",
        "b cd",
        " e fg",
        "hij kl ",
        " mn op ",
        "qrs tuv",
        " w  x yz"
    ];

    let expected = vec![
        vec!["a"],
        vec!["b", "cd"],
        vec!["e", "fg"],
        vec!["hij", "kl"],
        vec!["mn", "op"],
        vec!["qrs", "tuv"],
        vec!["w", "x", "yz"]
    ];

    scanning_template(inputs, expected);
}

#[test]
fn string_scanning() {
    let inputs = vec![
        "\"a\"",
        "b \"cd\"",
        " \"e fg\"",
        "\"hij\" \"kl\" ",
        " \"mn\" o \"p\" ",
        "\"qr\" s \"tu\" v",
        " \"w  \" x \"y\" z "
    ];

    let expected = vec![
        vec!["\"a\""],
        vec!["b", "\"cd\""],
        vec!["\"e fg\""],
        vec!["\"hij\"", "\"kl\""],
        vec!["\"mn\"", "o", "\"p\""],
        vec!["\"qr\"", "s", "\"tu\"", "v"],
        vec!["\"w  \"", "x", "\"y\"", "z"]
    ];

    scanning_template(inputs, expected);
}

#[test]
fn target_scanning() {
    let inputs = vec![
        "[a]",
        "b [cd]",
        " [e fg]",
        "[hij] [kl] ",
        " [mn] o [p] ",
        "[qr] s [tu] v",
        " [w  ] x [y] z "
    ];

    let expected = vec![
        vec!["[a]"],
        vec!["b", "[cd]"],
        vec!["[e fg]"],
        vec!["[hij]", "[kl]"],
        vec!["[mn]", "o", "[p]"],
        vec!["[qr]", "s", "[tu]", "v"],
        vec!["[w  ]", "x", "[y]", "z"]
    ];

    scanning_template(inputs, expected);
}

#[test]
fn monitor_scanning() {
    let inputs = vec![
        "{a}",
        "b {cd}",
        " {e fg}",
        "{hij} {kl} ",
        " {mn} o {p} ",
        "{qr} s {tu} v",
        " {w  } x {y} z "
    ];

    let expected = vec![
        vec!["{a}"],
        vec!["b", "{cd}"],
        vec!["{e fg}"],
        vec!["{hij}", "{kl}"],
        vec!["{mn}", "o", "{p}"],
        vec!["{qr}", "s", "{tu}", "v"],
        vec!["{w  }", "x", "{y}", "z"]
    ];

    scanning_template(inputs, expected);
}