package object

import "fmt"

type ObjectType string

type Object interface {
	Type() ObjectType
	Inspect() string
}

const (
	INTEGER_OBJ = "INTEGER"
    BOOLEAN_OBj = "BOOLEAN"
)

type Integer struct {
	Value int64
}

func (i *Integer) Inspect() string  { return fmt.Sprintf("%d", i.Value) }
func (i *Integer) Type() ObjectType { return INTEGER_OBJ }

type Boolean struct {
    Value bool 
}

func (b *Boolean) Inspect() string { return fmt.Sprintf("%t"m b.Value)}
func (b *Boolean) Type() ObjectType {return BOOLEAN_OBj}


