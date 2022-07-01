(* ::Package:: *)

implTuples[i_] := Block[
	{type, where, push},
	type = StringJoin@Table[TemplateApply["T`1`,", j], {j, 1, i}];
	where = StringJoin@Table[TemplateApply["Value: From<T`1`>,", j], {j, 1, i}];
	push = StringJoin@Table[TemplateApply["list.push(Value::from(v.`1`));", j - 1], {j, 1, i}];
	TemplateApply["
impl<`1`> From<(`1`)> for Value
where `2`
{
    fn from(v: (`1`)) -> Self {
        let mut list = Vec::with_capacity(`4`);
        `3`
        Value::from(list)
    }
}
",
		{type, where, push, i}
	]
];
head = "
use crate::Value;
use crate::value::List;

impl From<()> for Value {
    fn from(_: ()) -> Self {
        List::empty()
    }
}
";



Export[
	FileNameJoin[{NotebookDirectory[], "from_tuples.rs"}],
	head <> StringJoin[implTuples /@ Range[6]],
	"Text"
]
