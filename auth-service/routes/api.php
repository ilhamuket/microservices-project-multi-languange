<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\AuthController;
use App\Http\Middleware\CustomAuthenticate;

Route::post('/register', [AuthController::class, 'register']);
Route::post('/login', [AuthController::class, 'login']);
Route::post('/logout', [AuthController::class, 'logout'])->middleware(CustomAuthenticate::class.':api');
Route::get('/me', [AuthController::class, 'me'])->middleware(CustomAuthenticate::class.':api');