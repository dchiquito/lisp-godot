extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var invoker = $LispInvoker
onready var text = $TextEdit
onready var output_box = $Output
onready var error_box = $Error

# Called when the node enters the scene tree for the first time.
func _ready():
	invoker.connect("computation_complete", self, "computer_done")

func computer_done(output, error):
	print("is done!")
	print(output)
	print(error)
	output_box.text = output
	error_box.text = error



func _on_Button_pressed():
	print("pressin")
	invoker.parse(text.text)

