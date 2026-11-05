// Fixed observations for 12.5, not a parser. All AST values and CST entries,
// parents, depths, exact text and byte ranges checked against the native reader route.
export const infixExamples = [
  {
    "label": "Precedence",
    "source": "1 + 2 * 3",
    "ast": "(+ 1 (* 2 3))",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 9,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "BinaryExpr",
        "start": 0,
        "end": 9,
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
        "kind": "Whitespace",
        "start": 3,
        "end": 4,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "BinaryExpr",
        "start": 4,
        "end": 9,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 4,
        "end": 5,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 4,
        "end": 5,
        "depth": 4,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 5,
        "end": 6,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Star",
        "start": 6,
        "end": 7,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 7,
        "end": 8,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 8,
        "end": 9,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 8,
        "end": 9,
        "depth": 4,
        "parent": "NumberExpr"
      }
    ]
  },
  {
    "label": "Grouping",
    "source": "(1 + 2) * 3",
    "ast": "(* (group (+ 1 2)) 3)",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 11,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "BinaryExpr",
        "start": 0,
        "end": 11,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "GroupingExpr",
        "start": 0,
        "end": 7,
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
        "kind": "BinaryExpr",
        "start": 1,
        "end": 6,
        "depth": 3,
        "parent": "GroupingExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 1,
        "end": 2,
        "depth": 4,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 1,
        "end": 2,
        "depth": 5,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 2,
        "end": 3,
        "depth": 4,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Plus",
        "start": 3,
        "end": 4,
        "depth": 4,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 4,
        "end": 5,
        "depth": 4,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 5,
        "end": 6,
        "depth": 4,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 5,
        "end": 6,
        "depth": 5,
        "parent": "NumberExpr"
      },
      {
        "kind": "RightParen",
        "start": 6,
        "end": 7,
        "depth": 3,
        "parent": "GroupingExpr"
      },
      {
        "kind": "Whitespace",
        "start": 7,
        "end": 8,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Star",
        "start": 8,
        "end": 9,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 9,
        "end": 10,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 10,
        "end": 11,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 10,
        "end": 11,
        "depth": 3,
        "parent": "NumberExpr"
      }
    ]
  },
  {
    "label": "Left association",
    "source": "8 - 3 - 1",
    "ast": "(- (- 8 3) 1)",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 9,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "BinaryExpr",
        "start": 0,
        "end": 9,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "BinaryExpr",
        "start": 0,
        "end": 5,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 0,
        "end": 1,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 0,
        "end": 1,
        "depth": 4,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 1,
        "end": 2,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Minus",
        "start": 2,
        "end": 3,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 3,
        "end": 4,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 4,
        "end": 5,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 4,
        "end": 5,
        "depth": 4,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 5,
        "end": 6,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Minus",
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
        "kind": "NumberExpr",
        "start": 8,
        "end": 9,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 8,
        "end": 9,
        "depth": 3,
        "parent": "NumberExpr"
      }
    ]
  },
  {
    "label": "Spelling and comments",
    "source": " 001.0 // first\n + 02.00 * 3",
    "ast": "(+ 1 (* 2 3))",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 28,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "Whitespace",
        "start": 0,
        "end": 1,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "BinaryExpr",
        "start": 1,
        "end": 28,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "NumberExpr",
        "start": 1,
        "end": 6,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 1,
        "end": 6,
        "depth": 3,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 6,
        "end": 7,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "LineComment",
        "start": 7,
        "end": 15,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 15,
        "end": 17,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Plus",
        "start": 17,
        "end": 18,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 18,
        "end": 19,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "BinaryExpr",
        "start": 19,
        "end": 28,
        "depth": 2,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 19,
        "end": 24,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 19,
        "end": 24,
        "depth": 4,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 24,
        "end": 25,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Star",
        "start": 25,
        "end": 26,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Whitespace",
        "start": 26,
        "end": 27,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "NumberExpr",
        "start": 27,
        "end": 28,
        "depth": 3,
        "parent": "BinaryExpr"
      },
      {
        "kind": "Number",
        "start": 27,
        "end": 28,
        "depth": 4,
        "parent": "NumberExpr"
      }
    ]
  }
];
