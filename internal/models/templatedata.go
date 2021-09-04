package models

import "github.com/Aoi020608/bookings/internal/forms"

// TemplateData holds data sent from handlers to tempaltes
type TemplateData struct {
	StringMap       map[string]string
	IntMap          map[int]int
	FloatMap        map[float32]float32
	Data            map[string]interface{}
	CSRFToken       string
	Flash           string
	Warning         string
	Error           string
	Form            *forms.Form
	IsAuthenticated int
}
