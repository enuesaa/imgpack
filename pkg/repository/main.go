package repository

type Repos struct {
	Fs FsRepositoryInterface
}

func NewRepos() Repos {
	return Repos{
		Fs: &FsStorageRepository{},
	}
}

func NewLocalRepos() Repos {
	return Repos{
		Fs: &FsRepository{},
	}
}
