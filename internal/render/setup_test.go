package render

import (
	"encoding/gob"
	"net/http"
	"os"
	"testing"
	"time"

	"github.com/Aoi020608/bookings/internal/config"
	"github.com/Aoi020608/bookings/internal/models"
	"github.com/alexedwards/scs/v2"
)

var session *scs.SessionManager
var testApp config.AppConfig

func TestMain(m *testing.M) {
	// session
	gob.Register(models.Reservation{})
	// change this to true when in production
	testApp.InProduction = false

	session = scs.New()
	session.Lifetime = 24 * time.Hour
	session.Cookie.Persist = true
	session.Cookie.SameSite = http.SameSiteLaxMode
	session.Cookie.Secure = false

	testApp.Session = session

	app = &testApp

	os.Exit(m.Run())
}
