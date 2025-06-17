" Vim syntax file
" Language: FlowLang
" Maintainer: FlowLang Team
" Latest Revision: 2024

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword flowlangKeyword if then else end while for from to do return break continue
syn keyword flowlangDeclaration let def with
syn keyword flowlangOperator be and or not
syn keyword flowlangBuiltin show
syn keyword flowlangConstant true false null

" Comments
syn match flowlangComment "#.*$"

" Strings
syn region flowlangString start='"' end='"' contains=flowlangEscape
syn match flowlangEscape "\\\\\|\\\"" contained

" Numbers
syn match flowlangNumber "\<\d\+\>"
syn match flowlangFloat "\<\d\+\.\d\+\>"

" Operators
syn match flowlangArithOp "[+\-*/%]"
syn match flowlangCompOp "\(==\|!=\|<=\|>=\|<\|>\)"
syn match flowlangAssignOp "="

" Functions
syn match flowlangFunction "\<\w\+\>\s*("
syn match flowlangFunctionDef "\<def\s\+\zs\w\+\>"

" Highlighting
hi def link flowlangKeyword Keyword
hi def link flowlangDeclaration Type
hi def link flowlangOperator Operator
hi def link flowlangBuiltin Function
hi def link flowlangConstant Constant
hi def link flowlangComment Comment
hi def link flowlangString String
hi def link flowlangEscape SpecialChar
hi def link flowlangNumber Number
hi def link flowlangFloat Float
hi def link flowlangArithOp Operator
hi def link flowlangCompOp Operator
hi def link flowlangAssignOp Operator
hi def link flowlangFunction Function
hi def link flowlangFunctionDef Function

let b:current_syntax = "flowlang"