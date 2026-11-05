// Fixed observations for 12.6, not a parser. Every recovered CST entry, parent,
// depth, exact text, byte range and diagnostic checked against the native reader route.
export const recoveryExamples = [
  {
    "label": "Missing right",
    "source": "1 +",
    "ast": "no complete expression",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 3,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "BinaryExpr",
        "start": 0,
        "end": 3,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "NumberExpr",
        "start": 0,
        "end": 1,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 0,
        "end": 1,
        "depth": 3,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 1,
        "end": 2,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Plus",
        "start": 2,
        "end": 3,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "MissingExpr",
        "start": 3,
        "end": 3,
        "depth": 2,
        "parent": "BinaryExpr"
      }
    ],
    "errors": [
      {
        "message": "expected an expression",
        "start": 3,
        "end": 3
      }
    ]
  },
  {
    "label": "Missing left",
    "source": "* 2",
    "ast": "no complete expression",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 3,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "BinaryExpr",
        "start": 0,
        "end": 3,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "MissingExpr",
        "start": 0,
        "end": 0,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Star",
        "start": 0,
        "end": 1,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 1,
        "end": 2,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 2,
        "end": 3,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 2,
        "end": 3,
        "depth": 3,
        "parent": "NumberExpr"
      }
    ],
    "errors": [
      {
        "message": "expected an expression",
        "start": 0,
        "end": 1
      }
    ]
  },
  {
    "label": "Keep the close",
    "source": "(1 + )",
    "ast": "no complete expression",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 6,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "GroupingExpr",
        "start": 0,
        "end": 6,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "LeftParen",
        "start": 0,
        "end": 1,
        "depth": 2,
        "parent": "GroupingExpr"
      },
      {
        "kind": "BinaryExpr",
        "start": 1,
        "end": 5,
        "depth": 2,
        "parent": "GroupingExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 1,
        "end": 2,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 1,
        "end": 2,
        "depth": 4,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 2,
        "end": 3,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Plus",
        "start": 3,
        "end": 4,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 4,
        "end": 5,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "MissingExpr",
        "start": 5,
        "end": 5,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "RightParen",
        "start": 5,
        "end": 6,
        "depth": 2,
        "parent": "GroupingExpr"
      }
    ],
    "errors": [
      {
        "message": "expected an expression",
        "start": 5,
        "end": 6
      }
    ]
  },
  {
    "label": "Keep both errors",
    "source": "(1 @) + é",
    "ast": "no complete expression",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 10,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "BinaryExpr",
        "start": 0,
        "end": 10,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "GroupingExpr",
        "start": 0,
        "end": 5,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "LeftParen",
        "start": 0,
        "end": 1,
        "depth": 3,
        "parent": "GroupingExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 1,
        "end": 2,
        "depth": 3,
        "parent": "GroupingExpr"
      },
      {
        "kind": "Number",
        "start": 1,
        "end": 2,
        "depth": 4,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 2,
        "end": 3,
        "depth": 3,
        "parent": "GroupingExpr"
      },
      {
        "kind": "Error",
        "start": 3,
        "end": 4,
        "depth": 3,
        "parent": "GroupingExpr"
      },
      {
        "kind": "RightParen",
        "start": 4,
        "end": 5,
        "depth": 3,
        "parent": "GroupingExpr"
      },
      {
        "kind": "Whitespace",
        "start": 5,
        "end": 6,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Plus",
        "start": 6,
        "end": 7,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 7,
        "end": 8,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Error",
        "start": 8,
        "end": 10,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "MissingExpr",
        "start": 10,
        "end": 10,
        "depth": 2,
        "parent": "BinaryExpr"
      }
    ],
    "errors": [
      {
        "message": "invalid token",
        "start": 3,
        "end": 4
      },
      {
        "message": "invalid token",
        "start": 8,
        "end": 10
      }
    ]
  }
];
