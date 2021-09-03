package dbrepo

import (
	"errors"
	"time"

	"github.com/Aoi020608/bookings/internal/models"
)

func (m *testDBRepo) AllUsers() bool {
	return true
}

// insert reservation inserts a reservation into the database
func (m *testDBRepo) InsertReservation(res models.Reservation) (int, error) {
	// if  the room id is, then fail; otherwise, pass
	if res.RoomID == 2 {
		return 0, errors.New("some error")
	}
	return 1, nil
}

func (m *testDBRepo) InsertRoomRestriction(r models.RoomRestriction) error {
	if r.RoomID == 1000 {
		return errors.New("some error")
	}
	return nil
}

// searchavalilability returns true if availability exists for roomID and false if no availabiltiy exists
func (m *testDBRepo) SearchAvailabilityByRoomID(start, end time.Time, roomID int) (bool, error) {
	return false, nil
}

// SearchAvailabilityForAllRooms returns a slice of available rooms, if any for given date range
func (m *testDBRepo) SearchAvailabilityForAllRooms(start, end time.Time) ([]models.Room, error) {
	var rooms []models.Room
	return rooms, nil
}

func (m *testDBRepo) GetRoomByID(id int) (models.Room, error) {
	var room models.Room
	if id > 2 {
		return room, errors.New("some error")
	}
	return room, nil
}
