package main

import (
	"fmt"
	"testing"

	"github.com/Aoi020608/bookings/internal/config"
	"github.com/go-chi/chi/v5"
)

func TestRoutes(t *testing.T) {
	var app config.AppConfig

	mux := routes(&app)

	switch v := mux.(type) {
	case *chi.Mux:
	// do nothin: test passed
	default:
		t.Error(fmt.Sprintf("type is not *chi.Mux, type is %T", v)) // %T type of value
	}
}
