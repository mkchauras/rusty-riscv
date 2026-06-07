ENTRY(_start)
SECTIONS
{
	. = 0x80000000;

	.text.entry : ALIGN(4) {
		*(.text.entry)
	}

	.text : ALIGN(4) {
		*(.text)
		*(.text.*)
	}
}

