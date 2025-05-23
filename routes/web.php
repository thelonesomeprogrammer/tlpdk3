<?php

/**
 * All routes are defined here
 *
 * @category Routes
 * @package  TlpDk
 * @author   The Lonesome Programmer <m@thelonesomeprogrammer.dk>
 * @license  gpl-3.0 https://www.gnu.org/licenses/gpl-3.0.html
 * @link     https://thelonesomeprogrammer.dk
 */

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\VeiwController;
use App\Http\Controllers\ProjectController;
use App\Http\Controllers\ExperienceController;



Route::get('/', [VeiwController::class, 'index']);

Route::apiResource('api/projects', ProjectController::class);
Route::apiResource('api/experiences', ExperienceController::class);

Route::get('/{filename}', [VeiwController::class, 'file']);
Route::get('/{dir}/{filename}', [VeiwController::class, 'dirfile']);
