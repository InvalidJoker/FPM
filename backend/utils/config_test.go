package utils

import "testing"

func TestConfig(t *testing.T) {
	parsed := parseConfig("../test.fpm.config.toml")
	if parsed.Info.Name != "test" {
		t.Errorf("Expected 'test' got %s", parsed.Info.Name)
	}
}
