package dbrepo

import (
	"database/sql"

	"github.com/Aoi020608/bookings/internal/config"
	"github.com/Aoi020608/bookings/internal/repository"
)

type postgressDBRepo struct {
	App *config.AppConfig
	DB  *sql.DB
}

func NewPostgressRepo(conn *sql.DB, a *config.AppConfig) repository.DatabaseRepo {
	return &postgressDBRepo{
		App: a,
		DB:  conn,
	}
}
