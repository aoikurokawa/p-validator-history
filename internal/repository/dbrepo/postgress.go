package dbrepo

import "github.com/Aoi020608/bookings/internal/models"

func (m *postgressDBRepo) AllUsers() bool {
	return true
}

func (m *postgressDBRepo) InsertReservation(res models.Reservation) error {
	return nil
}
