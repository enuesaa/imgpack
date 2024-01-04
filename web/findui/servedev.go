//go:build dev

package findui

func init() {
	go RunDevCmd()
}

var Serve = ServeDev
