import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { OnInit } from '@angular/core';
import { HealthInfo } from '../models/health-info.model';
import { Game } from '../models/game.model';
import { ApiService } from './api.service';
import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { MatCardModule } from '@angular/material/card';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, HttpClientModule, MatCardModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit{
  constructor(private apiService: ApiService){}

  lastUpdated: Date = new Date();
  health: HealthInfo | null = null;
  games: Game[] = [
    {
      current: false,
      game_name: "Princess Peach Showtime!",
      completed: true,
      platform: "Nintendo Switch",
      difficulty: 4,
      rating: 7.9,
      est_hours: 15,
      game_art: "PrincessPeachShowtime.avif"
    },
    {
      current: true,
      game_name: "Super Mario Bros. U Deluxe",
      completed: false,
      platform: "Nintendo Switch",
      difficulty: 5,
      rating: 8.8,
      est_hours: 4,
      game_art: "SuperMarioBrosUDeluxe.avif"
    },
    {
      current: false,
      game_name: "Mario & Luigi Brothership",
      completed: false,
      platform: "Nintendo Switch",
      difficulty: 7.5,
      rating: 8.5,
      est_hours: 40,
      game_art: "MarioAndLuigiBrothership.avif"
    }
  ];

  ngOnInit(): void {
    this.apiService.getHealth().subscribe({
      next: (response) =>  this.health = response,
      error: (err) => console.error("Api call failed", err)
    });
  }

  // Get the current (featured) game
  get featuredGame(): Game | undefined {
    return this.games.find(game => game.current);
  }

  get pastGames(): Game[] {
    return this.games.filter(game => !game.current);
  }
}
