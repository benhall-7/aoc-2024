#[test]
fn test_key_pad() {
    use crate::day_21::{DirPad, NumPad};

    type B = DirPad;
    let ops_1 = NumPad::Button1.operations(NumPad::Button6);
    assert_eq!(
        ops_1,
        vec![
            [B::ButtonRight, B::ButtonRight, B::ButtonUp],
            [B::ButtonRight, B::ButtonUp, B::ButtonRight],
            [B::ButtonUp, B::ButtonRight, B::ButtonRight],
        ]
    );
    let ops_2 = NumPad::ButtonA.operations(NumPad::Button4);
    assert_eq!(
        ops_2,
        vec![
            [B::ButtonLeft, B::ButtonUp, B::ButtonLeft, B::ButtonUp],
            [B::ButtonLeft, B::ButtonUp, B::ButtonUp, B::ButtonLeft],
            [B::ButtonUp, B::ButtonLeft, B::ButtonLeft, B::ButtonUp],
            [B::ButtonUp, B::ButtonLeft, B::ButtonUp, B::ButtonLeft],
            [B::ButtonUp, B::ButtonUp, B::ButtonLeft, B::ButtonLeft]
        ]
    );
    let ops_3 = NumPad::Button7.operations(NumPad::Button0);
    assert_eq!(
        ops_3,
        vec![
            [B::ButtonRight, B::ButtonDown, B::ButtonDown, B::ButtonDown],
            [B::ButtonDown, B::ButtonRight, B::ButtonDown, B::ButtonDown],
            [B::ButtonDown, B::ButtonDown, B::ButtonRight, B::ButtonDown]
        ]
    );

    let case586a = NumPad::all_possible_operations(vec![
        NumPad::Button5,
        NumPad::Button8,
        NumPad::Button6,
        NumPad::ButtonA,
    ]);
    assert_eq!(
        case586a,
        vec![
            // A -> 5
            vec![
                vec![B::ButtonLeft, B::ButtonUp, B::ButtonUp, B::ButtonA],
                vec![B::ButtonUp, B::ButtonLeft, B::ButtonUp, B::ButtonA],
                vec![B::ButtonUp, B::ButtonUp, B::ButtonLeft, B::ButtonA]
            ],
            // 5 -> 8
            vec![vec![B::ButtonUp, B::ButtonA]],
            // 8 -> 6
            vec![
                vec![B::ButtonRight, B::ButtonDown, B::ButtonA],
                vec![B::ButtonDown, B::ButtonRight, B::ButtonA]
            ],
            // 6 -> A
            vec![vec![B::ButtonDown, B::ButtonDown, B::ButtonA]]
        ]
    );
}

#[test]
fn test_robot_pad() {
    use crate::day_21::DirPad;

    type B = DirPad;
    let ops_1 = B::ButtonDown.operations(B::ButtonA);
    assert_eq!(
        ops_1,
        vec![[B::ButtonRight, B::ButtonUp], [B::ButtonUp, B::ButtonRight],]
    );

    let ops_2 = B::ButtonLeft.operations(B::ButtonUp);
    assert_eq!(ops_2, vec![[B::ButtonRight, B::ButtonUp]]);

    let ops_3 = B::ButtonA.operations(B::ButtonLeft);
    assert_eq!(
        ops_3,
        vec![
            [B::ButtonLeft, B::ButtonDown, B::ButtonLeft],
            [B::ButtonDown, B::ButtonLeft, B::ButtonLeft],
        ]
    );

    let case586a = vec![
        // A -> 5
        vec![
            vec![B::ButtonLeft, B::ButtonUp, B::ButtonUp, B::ButtonA],
            vec![B::ButtonUp, B::ButtonLeft, B::ButtonUp, B::ButtonA],
            vec![B::ButtonUp, B::ButtonUp, B::ButtonLeft, B::ButtonA],
        ],
        // 5 -> 8
        vec![vec![B::ButtonUp, B::ButtonA]],
        // 8 -> 6
        vec![
            vec![B::ButtonRight, B::ButtonDown, B::ButtonA],
            vec![B::ButtonDown, B::ButtonRight, B::ButtonA],
        ],
        // 6 -> A
        vec![vec![B::ButtonDown, B::ButtonDown, B::ButtonA]],
    ];
    assert_eq!(
        case586a
            .into_iter()
            .map(|step| step
                .into_iter()
                .map(|possibility| DirPad::all_possible_operations(possibility))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        vec![
            // A -> 5
            vec![
                // 3 ways to get from A to 5
                // vec![B::ButtonLeft, B::ButtonUp, B::ButtonUp, B::ButtonA]
                vec![
                    vec![
                        // each of those moves may have one or more possibilities
                        vec![B::ButtonLeft, B::ButtonDown, B::ButtonLeft, B::ButtonA],
                        vec![B::ButtonDown, B::ButtonLeft, B::ButtonLeft, B::ButtonA]
                    ],
                    vec![vec![B::ButtonRight, B::ButtonUp, B::ButtonA]],
                    vec![vec![B::ButtonA]],
                    vec![vec![B::ButtonRight, B::ButtonA]]
                ],
                // vec![B::ButtonUp, B::ButtonLeft, B::ButtonUp, B::ButtonA]
                vec![
                    vec![vec![B::ButtonLeft, B::ButtonA]],
                    vec![vec![B::ButtonDown, B::ButtonLeft, B::ButtonA]],
                    vec![vec![B::ButtonRight, B::ButtonUp, B::ButtonA]],
                    vec![vec![B::ButtonRight, B::ButtonA]]
                ],
                // vec![B::ButtonUp, B::ButtonUp, B::ButtonLeft, B::ButtonA]
                vec![
                    vec![vec![B::ButtonLeft, B::ButtonA]],
                    vec![vec![B::ButtonA]],
                    vec![vec![B::ButtonDown, B::ButtonLeft, B::ButtonA]],
                    vec![
                        vec![B::ButtonRight, B::ButtonRight, B::ButtonUp, B::ButtonA],
                        vec![B::ButtonRight, B::ButtonUp, B::ButtonRight, B::ButtonA]
                    ]
                ]
            ],
            // 5 -> 8
            vec![
                // vec![B::ButtonUp, B::ButtonA]
                vec![
                    vec![vec![B::ButtonLeft, B::ButtonA]],
                    vec![vec![B::ButtonRight, B::ButtonA]]
                ]
            ],
            // 8 -> 6
            vec![
                // vec![B::ButtonRight, B::ButtonDown, B::ButtonA]
                vec![
                    vec![vec![B::ButtonDown, B::ButtonA]],
                    vec![vec![B::ButtonLeft, B::ButtonA]],
                    vec![
                        vec![B::ButtonRight, B::ButtonUp, B::ButtonA],
                        vec![B::ButtonUp, B::ButtonRight, B::ButtonA]
                    ]
                ],
                // vec![B::ButtonDown, B::ButtonRight, B::ButtonA]
                vec![
                    vec![
                        vec![B::ButtonLeft, B::ButtonDown, B::ButtonA],
                        vec![B::ButtonDown, B::ButtonLeft, B::ButtonA]
                    ],
                    vec![vec![B::ButtonRight, B::ButtonA]],
                    vec![vec![B::ButtonUp, B::ButtonA]]
                ]
            ],
            // 6 -> A
            vec![
                // vec![B::ButtonDown, B::ButtonDown, B::ButtonA]
                vec![
                    vec![
                        vec![B::ButtonLeft, B::ButtonDown, B::ButtonA],
                        vec![B::ButtonDown, B::ButtonLeft, B::ButtonA]
                    ],
                    vec![vec![B::ButtonA]],
                    vec![
                        vec![B::ButtonRight, B::ButtonUp, B::ButtonA],
                        vec![B::ButtonUp, B::ButtonRight, B::ButtonA]
                    ]
                ]
            ]
        ]
    );
}
